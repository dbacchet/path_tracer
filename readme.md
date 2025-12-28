# Path Tracer in Rust

**Version 1.1.0**

A simple path tracer implemented in Rust with real-time web visualization and concurrent viewer support.

This project implements a _very_ simple path tracer, done with the main purpose of learning Rust. I used the [Ray Tracing in a Weekend](http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html) series as a reference. I strongly recommend buying those books; they're a fantastic and very well written intro to ray tracing.

![sample image](sample_scene.png)

## Features

- **Path tracing** with physically-based rendering
- **Real-time web viewer** to watch rendering progress in your browser (NEW!)
- **Traditional desktop window** viewer
- **Concurrent viewers** - Run both window and web viewers simultaneously! (NEW!)
- **Multiple remote clients** - Multiple browsers can connect at once (NEW!)
- Multiple material types: Lambertian (diffuse), Metal, and Dielectric (glass)
- Depth of field / bokeh effects
- Progressive rendering with sample accumulation
- Live preview while rendering
- Comprehensive test suite (63 tests)

## Quick Start

### Both Viewers (Recommended!)

Run with both desktop window and web viewer simultaneously:

```bash
cargo run --release -- -o output.png --web
```

- Desktop window opens automatically
- Web viewer at: **http://localhost:3030**
- Multiple browsers can connect simultaneously
- Close window or press Ctrl+C to exit

### Web Viewer Only

Perfect for headless servers or remote viewing:

```bash
cargo run --release -- -o output.png --web --no-window
```

Then open your browser to: **http://localhost:3030**

- Multiple clients can connect
- See real-time client count
- Press Ctrl+C to exit

### Traditional Window Only

Run with just the desktop window viewer:

```bash
cargo run --release -- -o output.png
```

Press ESC to close the window early.

## Usage Examples

```bash
# Both viewers simultaneously (watch in window + browser)
cargo run --release -- -o output.png --web

# Web only with custom port
cargo run --release -- -o output.png --web --no-window -p 8080

# High quality render with both viewers
cargo run --release -- -o scene.png --web -w 1920 -h 1080 -s 500

# Quick test with both viewers
cargo run --release -- -o test.png --web -w 320 -h 180 -s 5

# Window only (original mode)
cargo run --release -- -o output.png -w 640 -h 360

# Test multiple clients (open multiple browser tabs!)
cargo run --release -- -o output.png --web --no-window
```

## Command Line Options

```
-o <NAME>          Output PNG filename (required)
-w <WIDTH>         Image width (default: 640)
-h <HEIGHT>        Image height (default: 360)
-s <SAMPLES>       Number of samples per pixel (default: 10)
-p <PORT>          Web server port (default: 3030)
--web              Enable web viewer (works with window)
--no-window        Disable desktop window viewer
--help             Print help menu
```

**Viewer Combinations:**
- `--web` alone: Both desktop window AND web viewer
- `--web --no-window`: Web viewer only
- No flags: Desktop window only (original mode)

**Note:** At least one viewer must be enabled!

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
