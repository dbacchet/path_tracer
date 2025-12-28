# Path Tracer in Rust

**Version 1.0.0**

A simple path tracer implemented in Rust with real-time web visualization.

This project implements a _very_ simple path tracer, done with the main purpose of learning Rust. I used the [Ray Tracing in a Weekend](http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html) series as a reference. I strongly recommend buying those books; they're a fantastic and very well written intro to ray tracing.

![sample image](sample_scene.png)

## Features

- **Path tracing** with physically-based rendering
- **Real-time web viewer** to watch rendering progress in your browser (NEW!)
- **Traditional desktop window** viewer (original mode)
- Multiple material types: Lambertian (diffuse), Metal, and Dielectric (glass)
- Depth of field / bokeh effects
- Progressive rendering with sample accumulation
- Live preview while rendering
- Comprehensive test suite (63 tests)

## Quick Start

### Web Viewer Mode (Recommended!)

Start the path tracer with the web viewer:

```bash
cargo run --release -- -o output.png --web
```

Then open your browser to: **http://localhost:3030**

You'll see the rendering progress in real-time with:
- Live image updates
- Current sample count
- Update rate (Hz)
- Pause/Resume controls

### Traditional Window Mode

Run with the original desktop window viewer:

```bash
cargo run --release -- -o output.png
```

Press ESC to close the window early.

## Usage Examples

```bash
# Quick test with web viewer
cargo run --release -- -o test.png --web -w 320 -h 180 -s 5

# High quality render
cargo run --release -- -o scene.png --web -w 1920 -h 1080 -s 500

# Custom port for web viewer
cargo run --release -- -o output.png --web -p 8080

# Custom resolution
cargo run --release -- -o output.png --web -w 800 -h 600
```

## Command Line Options

```
-o <NAME>          Output PNG filename (required)
-w <WIDTH>         Image width (default: 640)
-h <HEIGHT>        Image height (default: 360)
-s <SAMPLES>       Number of samples per pixel (default: 10)
-p <PORT>          Web server port (default: 3030)
--web              Enable web viewer instead of window
--help             Print help menu
```

## Running Tests

Run the comprehensive test suite:

```bash
cargo test
```

This includes **63 tests** covering:
- Vector mathematics (Vec3, Ray)
- Ray-sphere intersection
- Material scattering (Lambertian, Metal, Dielectric)
- Camera setup and ray generation
- Image rendering and gamma correction

## Project Architecture

- `src/pt_math.rs` - Vector and ray mathematics
- `src/objects.rs` - Sphere geometry and hit detection
- `src/material.rs` - Material properties and scattering
- `src/camera.rs` - Camera setup and ray generation
- `src/path_tracer.rs` - Main rendering loop and scene setup
- `src/web_viewer.rs` - Web server for live visualization (NEW!)
- `static/index.html` - Beautiful web UI with real-time updates (NEW!)

## Dependencies

- `tokio` - Async runtime for web server
- `warp` - Web framework for serving the live viewer
- `serde/serde_json` - JSON serialization for API
- `minifb` - Window display (desktop mode)
- `png` - PNG image encoding
- `rand` - Random number generation
- `indicatif` - Progress bar

## Scene

The default scene (inspired by Peter Shirley's "Ray Tracing in One Weekend") includes:
- A ground plane
- ~400 small spheres with random materials
- 3 large feature spheres (diffuse, glass, metal)

When you run the program it will generate the image shown above.

## Notes

Some notes about this project:
* The implementation is very naive (feedback is welcome!)
* No optimization has been implemented (yet)
* Live preview while rendering (both window and web modes)
* Real-time web viewer allows watching renders from any device

## Tips

- Start with low samples (5-10) to test quickly
- Higher samples = better quality but slower rendering
- Resolution affects memory and render time
- Use `--release` flag for 10x+ faster rendering
- Web viewer stays running after render completes (press Ctrl+C to exit)
- Works great on headless servers!

Enjoy watching your renders come to life! 🎨
