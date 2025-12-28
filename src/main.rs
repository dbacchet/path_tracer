mod pt_math;
mod camera;
mod material;
mod objects;
mod path_tracer;
mod web_viewer;

use pt_math::Vec3;
use camera::Camera;
use path_tracer::{Image, render_step, create_book_scene};
use objects::HitableList;
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
    opts.optflag("", "web", "enable web viewer instead of window");
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
    let use_web = matches.opt_present("web");
    
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           Path Tracer v{}                              ║", VERSION);
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Rendering scene...");
    // create empty image
    let mut image = Image::new(width, height);
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
    
    if use_web {
        // Web viewer mode
        println!("Starting in web viewer mode...");
        
        let shared_image = Arc::new(Mutex::new(image));
        let shared_image_clone = shared_image.clone();
        
        // Start web server in a separate task with abort handle
        let server_handle = tokio::spawn(async move {
            web_viewer::start_web_server(shared_image_clone, port).await;
        });
        
        // Give the web server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // Render loop
        use indicatif::ProgressBar;
        let bar = ProgressBar::new(samples as u64);
        bar.set_style(indicatif::ProgressStyle::default_bar()
                      .template("[{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta} rem.)")
                      .progress_chars("##-"));
        
        for _s in 0..samples {
            {
                let mut img = shared_image.lock().unwrap();
                render_step(&world, &camera, &mut img);
            }
            bar.inc(1);
            // Small delay to allow web requests to be processed
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        bar.finish();
        
        println!("...Done!");
        println!("Rendering complete. Image saved to {}", output_filename);
        
        // Save final image
        let final_image = shared_image.lock().unwrap();
        final_image.save(&output_filename);
        drop(final_image); // Release the lock
        
        // Keep server running
        println!("Web server still running at http://localhost:{}", port);
        println!("Press Ctrl+C to exit.");
        
        // Wait for Ctrl+C with a timeout-based approach
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("\nShutting down gracefully...");
            }
            _ = server_handle => {
                println!("\nServer stopped unexpectedly.");
            }
        }
        
        println!("Goodbye!");
        
    } else {
        // Original window mode
        create_window_viewer(&world, &camera, &mut image, samples, &output_filename);
    }
}

fn create_window_viewer(
    world: &HitableList,
    camera: &Camera,
    image: &mut Image,
    samples: u32,
    output_filename: &str,
) {
    use minifb::{Key, WindowOptions, Window};
    use indicatif::ProgressBar;
    
    // create window with live framebuffer 
    let mut buffer: Vec<u32> = vec![0; (image.width * image.height) as usize];
    let mut window = Window::new("Test - ESC to exit",
                                 image.width as usize,
                                 image.height as usize,
                                 WindowOptions::default()).unwrap_or_else(|e| { panic!("{}", e); });
    // progress bar
    let bar = ProgressBar::new(samples as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar()
                  .template("[{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta} rem.)")
                  .progress_chars("##-"));
    // render image and update  window
    for _s in 0..samples {
        if window.is_open() && !window.is_key_down(Key::Escape) {
            render_step(&world, &camera, image);
            // update framebuffer
            for j in 0..image.height {
                for i in 0..image.width {
                    let (r,g,b) = image.val_rgb(i,j);
                    let rgb: u32 = 0xff << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
                    buffer[(i+j*image.width) as usize] = rgb;
                }
            }
            window.update_with_buffer(&buffer).unwrap();
        }
        bar.inc(1);
    }
    bar.finish();

    println!("...Done!");
    // save image to file
    image.save(&output_filename);
}

