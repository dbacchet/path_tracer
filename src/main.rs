// extern crate png;
//
mod pt_math;
use pt_math::{Vec3, Ray, mul_component};

mod objects;
use objects::{Hitable, Sphere, HitableList};

mod camera;
use camera::Camera;

mod material;
use material::{Lambertian, Metal, Dielectric};

use rand::Rng;

struct Image {
    width: u32,
    height: u32,
    data: Vec<Vec3>,
}

impl Image {
    fn new(width: u32, height: u32) -> Image {
        let mut data: Vec<Vec3> = Vec::new();
        data.resize((width*height) as usize, Vec3::new(0.0,0.0,0.0));
        Image {
            width: width,
            height: height,
            // data: vec![Vec3{x:0.0, y:0.0, z:0.0}; (width*height) as usize], // RGB pixels
            data: data,
        }
    }

    fn save(&self, filename: &str) {
        let file = std::fs::File::create(filename).unwrap();
        let ref mut w = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        // image data rows start from top. need to swap lines
        let mut raw_data: Vec<u8> = vec![0; (self.width*self.height*3) as usize];
        for (i, &v) in self.data.iter().enumerate() {
            let row = i as u32 / self.width;
            let col = i as u32 % self.width;
            let idx = (self.height-row-1)*self.width + col;
            let idx = idx as usize;
            raw_data[3*idx+0] = (v.x*255.99) as u8;
            raw_data[3*idx+1] = (v.y*255.99) as u8;
            raw_data[3*idx+2] = (v.z*255.99) as u8;
        }
        writer.write_image_data(&raw_data).unwrap(); // Save
    }
}

fn color<T: Hitable>(ray: Ray, world: &T, depth: i32) -> Vec3 {
    const MIN_DIST : f32 = 0.0001; //10.0 * std::f32::MIN_POSITIVE;
    const MAX_ITX: i32 = 50;
    let max_dist = 1000000.0;
    if let Some(hitrecord) = world.hit(&ray, MIN_DIST, max_dist) {
        if depth >= MAX_ITX {
            return Vec3::new(0.0,0.0,0.0);
        }
        if let Some(scatter) = hitrecord.material.scatter(ray, hitrecord.point, hitrecord.normal) {
            return mul_component(scatter.color, color(scatter.ray, world, depth+1));
        } else {
            return Vec3::new(0.0,0.0,0.0);
        }
    } else {
        let unit_dir = pt_math::unit_vector(&ray.direction);
        let t = 0.5 * (unit_dir.y + 1.0);
        Vec3::new(1.0,1.0,1.0)*(1.0-t) + Vec3::new(0.5,0.7,1.0)*t
    }
}

fn main() {
    println!("sample path tracing. Rendering scene...");
    // image and camera data
    let width = 400;
    let height = 200;
    let samples = 100;
    // let width = 1280;
    // let height = 640;
    let mut image = Image::new(width, height);
    let camera = Camera::new();
    // create scene
    let mut world = HitableList::new();
    world.add(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.8,0.3,0.3)))));
    world.add(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8,0.8,0.0)))));
    world.add(Sphere::new(Vec3::new(1.0,0.0,-1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8,0.6,0.2), 0.1))));
    world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, Box::new(Dielectric::new(1.1, 0.0))));
    world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), -0.45, Box::new(Dielectric::new(1.1, 0.0))));
    // world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8,0.8,0.8), 0.5))));
    world.add(Sphere::new(Vec3::new(-1.0,0.5,-1.0), 0.15, Box::new(Metal::new(Vec3::new(0.1,0.1,0.1), 0.0))));
    // fill image
    let mut rng = rand::thread_rng();
    for j in 0..height {
        for i in 0..width {
            let mut col = Vec3::new(0.0,0.0,0.0);
            for _s in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / (width as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (height as f32);
                let ray = camera.get_ray(u,v);
                col = col + color(ray, &world, 0);
            }
            col = col / samples as f32;
            // gamma correct using "gamma 2"
            let col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            image.data[(j*width+i) as usize] = col;
        }
    }
    println!("...Done!");
    // save image to file
    image.save("ciccio.png");
}

