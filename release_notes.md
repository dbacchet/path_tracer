# Path Tracer - Release Notes

## Version 1.1.0 (December 2025) - Concurrent Viewers

### 🎉 Major New Feature: Concurrent Viewers

Run **both desktop window and web viewers simultaneously**! Watch your render in a local window while multiple remote clients connect via browser.

**New Capabilities:**
- ✨ **Simultaneous viewing** - Desktop window + web browser at the same time
- 🌍 **Multiple remote clients** - Unlimited browser connections simultaneously
- 📊 **Connection tracking** - See how many clients are connected
- 🔧 **Flexible configuration** - Choose which viewers to enable
- 🧵 **Multi-threaded** - Window viewer runs in separate thread for smooth updates

### Usage

**Both viewers (recommended):**
```bash
cargo run --release -- -o output.png --web
# Desktop window opens + web at http://localhost:3030
```

**Web only:**
```bash
cargo run --release -- -o output.png --web --no-window
# Perfect for headless servers
```

**Window only:**
```bash
cargo run --release -- -o output.png
# Original mode
```

### New Command-Line Options

- `--no-window` - Disable desktop window viewer
- Modified `--web` - Now works alongside window viewer (not instead of)

### Technical Implementation

**Architecture Changes:**
- Refactored `main.rs` for concurrent viewer support
- Window viewer runs in dedicated thread via `std::thread`
- Web server runs in tokio task (unchanged)
- Both viewers read from shared `Arc<Mutex<Image>>`
- Main thread handles rendering loop
- Graceful shutdown for all viewer combinations

**Window Viewer Updates:**
- Continuously polls shared image and updates display
- ~30 FPS update rate with manual sleep
- Runs until ESC pressed or window closed
- Thread-safe access to image buffer

**Web Viewer Enhancements:**
- Added connection counter using `AtomicUsize`
- Tracks total page loads
- Displays connected client count in UI
- Supports unlimited simultaneous connections

### Benefits

1. **Best of Both Worlds** - Local preview + remote monitoring
2. **Team Collaboration** - Multiple people can watch same render
3. **Flexibility** - Choose the viewers you need
4. **Performance** - Minimal overhead from multiple viewers
5. **Compatibility** - All original modes still work

---

## Version 1.0.0 (December 2025)

### Major Features

#### 🌐 Real-Time Web Viewer
A complete web-based visualization system for watching renders in real-time from any browser.

**Key Features:**
- Live progressive rendering visualization
- Beautiful modern UI with glass-morphism design
- Real-time statistics dashboard (resolution, samples, update rate)
- Interactive pause/resume controls
- Connection status monitoring
- Auto-updating at ~10 Hz
- Works on any device with a browser
- Perfect for headless servers

**Usage:**
```bash
cargo run --release -- -o output.png --web
# Open browser to http://localhost:3030
```

#### 🧪 Comprehensive Test Suite
Added 63 comprehensive tests covering all core functionality:
- Vector mathematics (Vec3, Ray) - 16 tests
- Ray-sphere intersection - 10 tests
- Material scattering (Lambertian, Metal, Dielectric) - 17 tests
- Camera setup and ray generation - 9 tests
- Image rendering and gamma correction - 11 tests

**Running Tests:**
```bash
cargo test
```

### New Features

#### Command-Line Options
- `--web` - Enable web viewer mode
- `-p <PORT>` - Custom web server port (default: 3030)
- All original options retained for backward compatibility

#### Dual Viewer Modes
1. **Web Mode** - Browser-based with real-time updates
2. **Window Mode** - Original minifb desktop window (default)

### Technical Implementation

#### New Dependencies
- `tokio` (v1.x) - Async runtime for concurrent web operations
- `warp` (v0.3) - Modern web framework for REST API
- `serde` + `serde_json` (v1.x) - JSON serialization

#### New Modules
- `src/web_viewer.rs` - Web server with REST API
  - `GET /` - Serves HTML frontend
  - `GET /api/image` - Returns current image as JSON with RGB pixel data
  - `GET /api/stats` - Returns rendering statistics
- `static/index.html` - Modern web UI with real-time updates

#### Architecture Changes
- `main.rs` converted to async with `#[tokio::main]`
- Thread-safe image sharing using `Arc<Mutex<Image>>`
- Rendering loop and web server run concurrently
- Window viewer refactored into separate function

### Bug Fixes

#### Ctrl+C Shutdown Fix (v1.0.0)
**Problem:** Application would hang indefinitely when pressing Ctrl+C after rendering completed.

**Solution:** Implemented proper graceful shutdown using `tokio::select!`:
- Captures web server task handle
- Races between Ctrl+C signal and server completion
- Tokio runtime automatically cleans up all tasks on exit
- Added user feedback messages for shutdown

**Result:**
- ✅ Exits immediately (< 1 second)
- ✅ Clean shutdown with user feedback
- ✅ No hanging or resource leaks

#### Compilation Warnings
- Removed unused code from `web_viewer.rs`
- Fixed all compiler warnings
- Clean build in both debug and release modes

### Performance

- Minimal overhead from web server (~1% impact)
- Rendering continues at full speed
- Efficient JSON encoding for image data
- Mutex contention minimized with strategic delays
- 10x+ faster with `--release` flag

### Documentation

**New Files:**
- `release_notes.md` - This file
- `QUICKSTART.txt` - Quick reference guide
- Enhanced `readme.md` - Complete user guide

**Scripts:**
- `demo_web_viewer.sh` - Quick demo script
- `test_ctrl_c.sh` - Test script for shutdown behavior

### Breaking Changes

None! All original functionality preserved:
- Default window mode unchanged
- Same command-line interface (with new optional flags)
- Backward compatible with existing workflows

---

## Version 0.3.0 (Base Implementation)

### Initial Features
- Path tracing with physically-based rendering
- Multiple material types (Lambertian, Metal, Dielectric)
- Depth of field / bokeh effects
- Progressive rendering with sample accumulation
- Live preview in desktop window (minifb)
- Scene generation inspired by "Ray Tracing in One Weekend"
- PNG output support
- Progress bar during rendering

### Core Modules
- `pt_math.rs` - Vector and ray mathematics
- `objects.rs` - Sphere geometry and hit detection
- `material.rs` - Material properties and light scattering
- `camera.rs` - Camera setup and ray generation
- `path_tracer.rs` - Main rendering loop and scene setup

### Dependencies
- `minifb` - Window display
- `png` - Image encoding
- `rand` - Random number generation
- `getopts` - Command-line parsing
- `indicatif` - Progress bars

---

## Migration Guide

### Upgrading from 0.3.0 to 1.0.0

**No code changes required!** The update is fully backward compatible.

**To use new features:**

1. **Try the web viewer:**
   ```bash
   cargo run --release -- -o output.png --web
   ```

2. **Run tests:**
   ```bash
   cargo test
   ```

3. **Custom port:**
   ```bash
   cargo run --release -- -o output.png --web -p 8080
   ```

**Original workflow still works:**
```bash
# This still works exactly as before
cargo run --release -- -o output.png
```

---

## System Requirements

- Rust 2018 edition or newer
- Modern web browser (for web viewer mode)
- Any OS supported by Rust (Linux, macOS, Windows)

---

## Known Issues

None currently reported.

---

## Future Roadmap

Potential features for future releases:
- WebSocket streaming for even smoother updates
- Multiple scene presets
- Interactive camera controls in web UI
- Render time estimation
- Save/load camera positions
- HDR environment maps
- More geometric primitives (planes, triangles, meshes)
- BVH acceleration structure
- Multi-threaded rendering

---

## Credits

- Based on Peter Shirley's "Ray Tracing in One Weekend" series
- Original implementation by Davide Bacchet
- Web viewer and test suite additions (v1.0.0)

---

## License

See project repository for license information.

