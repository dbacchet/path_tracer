use std::sync::{Arc, Mutex};
use warp::Filter;
use crate::path_tracer::Image;

pub async fn start_web_server(shared_image: Arc<Mutex<Image>>, port: u16) {
    let shared_image = warp::any().map(move || shared_image.clone());

    // Serve the HTML page
    let index = warp::path::end()
        .map(|| warp::reply::html(include_str!("../static/index.html")));

    // API endpoint to get current image data
    let image_api = warp::path("api")
        .and(warp::path("image"))
        .and(warp::path::end())
        .and(shared_image.clone())
        .map(|image: Arc<Mutex<Image>>| {
            let img = image.lock().unwrap();
            let json = image_to_json(&img);
            warp::reply::json(&json)
        });

    // API endpoint to get image statistics
    let stats_api = warp::path("api")
        .and(warp::path("stats"))
        .and(warp::path::end())
        .and(shared_image)
        .map(|image: Arc<Mutex<Image>>| {
            let img = image.lock().unwrap();
            let stats = serde_json::json!({
                "width": img.width,
                "height": img.height,
                "samples": img.samples,
            });
            warp::reply::json(&stats)
        });

    let routes = index.or(image_api).or(stats_api);

    println!("Web viewer starting at http://localhost:{}", port);
    println!("Open your browser to view the rendering in real-time!");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

fn image_to_json(image: &Image) -> serde_json::Value {
    let mut pixels = Vec::new();
    
    for j in 0..image.height {
        for i in 0..image.width {
            let (r, g, b) = image.val_rgb(i, j);
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
        }
    }

    serde_json::json!({
        "width": image.width,
        "height": image.height,
        "samples": image.samples,
        "pixels": pixels,
    })
}

