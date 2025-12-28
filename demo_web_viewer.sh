#!/bin/bash

echo "================================================"
echo "Path Tracer - Web Viewer Demo"
echo "================================================"
echo ""
echo "This will start a path tracer with web viewer."
echo "Open your browser to: http://localhost:3030"
echo ""
echo "Press Ctrl+C to stop when rendering is complete."
echo ""
echo "Starting in 3 seconds..."
sleep 3

# Run with smaller resolution and fewer samples for quick demo
cargo run --release -- -o demo_output.png --web -w 400 -h 300 -s 20

echo ""
echo "================================================"
echo "Demo complete! Check demo_output.png"
echo "================================================"

