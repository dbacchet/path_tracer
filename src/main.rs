mod pt_math;
mod camera;
mod material;
mod objects;
mod path_tracer;

use pt_math::Vec3;
use camera::Camera;
use path_tracer::{Image, render_step, create_book_scene};

extern crate getopts;
use getopts::Options;
extern crate minifb;
use minifb::{Key, WindowOptions, Window};
extern crate indicatif;
use indicatif::ProgressBar;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // parse command line options
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.reqopt("o", "", "set output file name", "NAME");
    opts.optopt("w", "width", "image width (default=640)", "");
    opts.optopt("h", "height", "image height (default=360)", "");
    opts.optopt("s", "samples", "number of samples (default=10)", "");
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
    println!("sample path tracing. Rendering scene...");
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
    // create window with live framebuffer 
    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];
    let mut window = Window::new("Test - ESC to exit",
                                 width as usize,
                                 height as usize,
                                 WindowOptions::default()).unwrap_or_else(|e| { panic!("{}", e); });
    // progress bar
    let bar = ProgressBar::new(samples as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar()
                  .template("[{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta} rem.)")
                  .progress_chars("##-"));
    // render image and update  window
    for _s in 0..samples {
        if window.is_open() && !window.is_key_down(Key::Escape) {
            render_step(&world, &camera, &mut image);
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

