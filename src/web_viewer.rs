use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use warp::Filter;
use crate::path_tracer::Image;

pub async fn start_web_server(shared_image: Arc<Mutex<Image>>, port: u16) {
    let connection_count = Arc::new(AtomicUsize::new(0));
    
    let shared_image_filter = warp::any().map(move || shared_image.clone());
    let connection_counter = warp::any().map({
        let counter = connection_count.clone();
        move || counter.clone()
    });

    // Serve the HTML page
    let index = warp::path::end()
        .and(connection_counter.clone())
        .map(|counter: Arc<AtomicUsize>| {
            counter.fetch_add(1, Ordering::SeqCst);
            warp::reply::html(include_str!("../static/index.html"))
        });

    // API endpoint to get current image data
    let image_api = warp::path("api")
        .and(warp::path("image"))
        .and(warp::path::end())
        .and(shared_image_filter.clone())
        .map(|image: Arc<Mutex<Image>>| {
            let img = image.lock().unwrap();
            let json = image_to_json(&img);
            warp::reply::json(&json)
        });

    // API endpoint to get image statistics
    let stats_api = warp::path("api")
        .and(warp::path("stats"))
        .and(warp::path::end())
        .and(shared_image_filter)
        .and(connection_counter)
        .map(|image: Arc<Mutex<Image>>, counter: Arc<AtomicUsize>| {
            let img = image.lock().unwrap();
            let stats = serde_json::json!({
                "width": img.width,
                "height": img.height,
                "samples": img.samples,
                "connections": counter.load(Ordering::SeqCst),
            });
            warp::reply::json(&stats)
        });

    let routes = index.or(image_api).or(stats_api);

    println!("Web viewer starting at http://localhost:{}", port);
    println!("Multiple clients can connect simultaneously!");
    
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

