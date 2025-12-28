mod pt_math;
mod camera;
mod material;
mod objects;
mod path_tracer;
mod web_viewer;

use pt_math::Vec3;
use camera::Camera;
use path_tracer::{Image, render_step, create_book_scene};
use std::sync::{Arc, Mutex};

extern crate getopts;
use getopts::Options;
extern crate minifb;
extern crate indicatif;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

#[tokio::main]
async fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    let args: Vec<String> = std::env::args().collect();

    // parse command line options
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.reqopt("o", "", "set output file name", "NAME");
    opts.optopt("w", "width", "image width (default=640)", "");
    opts.optopt("h", "height", "image height (default=360)", "");
    opts.optopt("s", "samples", "number of samples (default=10)", "");
    opts.optopt("p", "port", "web server port (default=3030)", "");
    opts.optflag("", "web", "enable web viewer (can run with window)");
    opts.optflag("", "no-window", "disable desktop window viewer");
    opts.optflag("", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { println!("{}", f.to_string());
                    print_usage(&program, opts); 
                    return;
        }
    };
    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }
    let output_filename = matches.opt_str("o").unwrap_or(String::from("image.png"));
    let width = matches.opt_get_default::<u32>("w", 640).expect("invalid width");
    let height = matches.opt_get_default::<u32>("h", 360).expect("invalid heigh");
    let samples = matches.opt_get_default::<u32>("s", 10).expect("invalid number of samples");
    let port = matches.opt_get_default::<u16>("p", 3030).expect("invalid port");
    let enable_web = matches.opt_present("web");
    let enable_window = !matches.opt_present("no-window");
    
    // Validate: at least one viewer must be enabled
    if !enable_web && !enable_window {
        eprintln!("Error: Cannot disable all viewers. Remove --no-window or add --web.");
        return;
    }
    
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           Path Tracer v{}                              ║", VERSION);
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    
    // Show which viewers are enabled
    if enable_web && enable_window {
        println!("📺 Desktop window viewer: ENABLED");
        println!("🌐 Web viewer: ENABLED at http://localhost:{}", port);
        println!("   (Multiple clients can connect simultaneously)");
    } else if enable_web {
        println!("🌐 Web viewer: ENABLED at http://localhost:{}", port);
        println!("📺 Desktop window viewer: DISABLED");
    } else {
        println!("📺 Desktop window viewer: ENABLED");
        println!("🌐 Web viewer: DISABLED");
    }
    println!();
    println!("Rendering scene...");
    // create empty image
    let image = Image::new(width, height);
    // camera
    let aperture = 0.051;
    let lookfrom = Vec3::new(10.0, 1.8, 2.4);
    let lookat = Vec3::new(0.0, 0.0, 0.5);
    let up = Vec3::new(0.0,1.0,0.0); 
    let dist_to_focus = (lookfrom-Vec3::new(4.0, 1.0, 0.0)).length();
    let camera = Camera::new(lookfrom, lookat, up, 
                             30.0, (width as f32)/(height as f32),
                             aperture, dist_to_focus);
    // create scene
    let world = create_book_scene();
    // let world = create_test_scene();
    
    // Use shared image for concurrent access by multiple viewers
    let shared_image = Arc::new(Mutex::new(image));
    
    // Start web server if enabled
    let server_handle = if enable_web {
        let shared_image_clone = shared_image.clone();
        println!("Starting web server...");
        let handle = tokio::spawn(async move {
            web_viewer::start_web_server(shared_image_clone, port).await;
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        println!("Web server ready at http://localhost:{}", port);
        Some(handle)
    } else {
        None
    };
    
    // Start window viewer in separate thread if enabled
    let window_handle = if enable_window {
        let shared_image_clone = shared_image.clone();
        let img_width = width;
        let img_height = height;
        
        Some(std::thread::spawn(move || {
            run_window_viewer(shared_image_clone, img_width, img_height);
        }))
    } else {
        None
    };
    
    // Main rendering loop
    use indicatif::ProgressBar;
    let bar = ProgressBar::new(samples as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar()
                  .template("[{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta} rem.)")
                  .progress_chars("##-"));
    
    println!();
    for _s in 0..samples {
        {
            let mut img = shared_image.lock().unwrap();
            render_step(&world, &camera, &mut img);
        }
        bar.inc(1);
        // Small delay to allow viewers to update
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
    bar.finish();
    
    println!();
    println!("...Done!");
    println!("Rendering complete. Image saved to {}", output_filename);
    
    // Save final image
    let final_image = shared_image.lock().unwrap();
    final_image.save(&output_filename);
    drop(final_image);
    
    // Handle shutdown based on what's running
    if enable_web && enable_window {
        println!();
        println!("Both viewers still running.");
        println!("  - Close the window or press Ctrl+C to exit.");
        
        // Wait for either window to close or Ctrl+C
        if let Some(handle) = window_handle {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("\nShutting down gracefully...");
                }
                _ = tokio::task::spawn_blocking(move || handle.join()) => {
                    println!("\nWindow closed.");
                }
            }
        }
    } else if enable_web {
        println!();
        println!("Web server still running at http://localhost:{}", port);
        println!("Press Ctrl+C to exit.");
        
        if let Some(server) = server_handle {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("\nShutting down gracefully...");
                }
                _ = server => {
                    println!("\nServer stopped unexpectedly.");
                }
            }
        }
    } else if enable_window {
        println!();
        println!("Close the window to exit.");
        
        // Wait for window to close
        if let Some(handle) = window_handle {
            let _ = handle.join();
        }
    }
    
    println!("Goodbye!");
}

fn run_window_viewer(shared_image: Arc<Mutex<Image>>, width: u32, height: u32) {
    use minifb::{Key, WindowOptions, Window};
    
    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];
    let mut window = Window::new(
        "Path Tracer - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default()
    ).unwrap_or_else(|e| { panic!("{}", e); });
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update framebuffer from shared image
        {
            let img = shared_image.lock().unwrap();
            for j in 0..img.height {
                for i in 0..img.width {
                    let (r, g, b) = img.val_rgb(i, j);
                    let rgb: u32 = 0xff << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
                    buffer[(i + j * img.width) as usize] = rgb;
                }
            }
        }
        
        window.update_with_buffer(&buffer).unwrap();
        
        // Small delay to avoid consuming too much CPU
        std::thread::sleep(std::time::Duration::from_millis(33)); // ~30 FPS
    }
}

