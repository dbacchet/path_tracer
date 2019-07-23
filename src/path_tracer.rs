use crate::pt_math::{Vec3, Ray, mul_component, unit_vector};
use crate::objects::{Hitable, Sphere, HitableList};
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};

use rand::Rng;
use rand::prelude::*;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec3>,
    pub samples: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let mut data: Vec<Vec3> = Vec::new();
        data.resize((width*height) as usize, Vec3::new(0.0,0.0,0.0));
        Image {
            width: width,
            height: height,
            data: data,
            samples: 0
        }
    }

    pub fn save(&self, filename: &str) {
        let file = std::fs::File::create(filename).unwrap();
        let ref mut w = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        // image data rows start from top. need to swap lines
        let mut raw_data: Vec<u8> = vec![0; (self.width*self.height*3) as usize];
        for i in 0..self.data.len() {
            let row = i as u32 / self.width;
            let col = i as u32 % self.width;
            let idx = (self.height-row-1)*self.width + col;
            let idx = idx as usize;
            let v = self.val(i);
            raw_data[3*idx+0] = (v.x*255.99) as u8;
            raw_data[3*idx+1] = (v.y*255.99) as u8;
            raw_data[3*idx+2] = (v.z*255.99) as u8;
        }
        writer.write_image_data(&raw_data).unwrap(); // Save
    }
    
    pub fn val(&self, i: usize) -> Vec3 {
        let col = self.data[i]/(self.samples as f32);
        // gamma correct using "gamma 2"
        Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt())
    }

    pub fn val_rgb(&self, i: u32, j: u32) -> (u8, u8, u8) {
        let idx = ((self.height-j-1)*self.width + i) as usize;
        let v = self.val(idx);
        let r = (v.x*255.99) as u8;
        let g = (v.y*255.99) as u8;
        let b = (v.z*255.99) as u8;
        (r,g,b)
        // let rgba: u32 = 0xff << 24 | r << 16 | g << 8 | b;
        // return rgba;
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
        let unit_dir = unit_vector(ray.direction);
        let t = 0.5 * (unit_dir.y + 1.0);
        Vec3::new(1.0,1.0,1.0)*(1.0-t) + Vec3::new(0.5,0.7,1.0)*t
    }
}

#[allow(dead_code)]
pub fn create_test_scene() -> HitableList {
    let mut world = HitableList::new();
    world.add(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.8,0.3,0.3)))));
    world.add(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8,0.8,0.0)))));
    world.add(Sphere::new(Vec3::new(1.0,0.0,-1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8,0.6,0.2), 0.1))));
    world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, Box::new(Dielectric::new(1.1, 0.0))));
    world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), -0.45, Box::new(Dielectric::new(1.1, 0.0))));
    // world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8,0.8,0.8), 0.5))));
    world.add(Sphere::new(Vec3::new(-1.0,0.5,-1.0), 0.15, Box::new(Metal::new(Vec3::new(0.1,0.1,0.1), 0.0))));
    world
}

pub fn create_book_scene() -> HitableList {
    let mut rng = SmallRng::seed_from_u64(123123123123<<10); // deterministic RNG to always build the same scene
    let mut world = HitableList::new();
    // giant sphere for the floor
    world.add(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5,0.5,0.5)))));
    // a bunch of small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new((a as f32)+0.9*rng.gen::<f32>(),0.2,(b as f32)+0.9*rng.gen::<f32>());
            if (center - Vec3::new(4.0,0.2,0.0)).length() >0.9 {
                if choose_mat<0.8 { // diffuse
                    world.add(Sphere::new(center, 0.2, Box::new(Lambertian::new(Vec3::new(rng.gen::<f32>()*rng.gen::<f32>(),
                                                                                          rng.gen::<f32>()*rng.gen::<f32>(),
                                                                                          rng.gen::<f32>()*rng.gen::<f32>())))));
                } else if choose_mat<0.95 {
                    world.add(Sphere::new(center, 0.2, Box::new(Metal::new(Vec3::new(0.5*(rng.gen::<f32>()+1.0),
                                                                                     0.5*(rng.gen::<f32>()+1.0),
                                                                                     0.5*(rng.gen::<f32>()+1.0)),
                                                                           0.5*rng.gen::<f32>()))));

                } else {
                    world.add(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5, 0.1*rng.gen::<f32>()))));
                }
            }
        }
    }
    // add the 3 main spheres
    world.add(Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4,0.2,0.1)))));
    world.add(Sphere::new(Vec3::new( 0.0,1.0,0.0), 1.0, Box::new(Dielectric::new(1.5,0.0))));
    world.add(Sphere::new(Vec3::new( 4.0,1.0,0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7,0.6,0.5), 0.0))));

    world
}

#[allow(dead_code)]
pub fn render(world: &HitableList, camera: &Camera, image: &mut Image, samples: u32) {
    // fill image
    for _s in 0..samples {
        render_step(world, camera, image);
    }
}

pub fn render_step(world: &HitableList, camera: &Camera, image: &mut Image) {
    // fill image
    let mut rng = rand::thread_rng();
    for j in 0..image.height {
        for i in 0..image.width {
            let u = (i as f32 + rng.gen::<f32>()) / (image.width as f32);
            let v = (j as f32 + rng.gen::<f32>()) / (image.height as f32);
            let ray = camera.get_ray(u,v);
            let mut col = Vec3::new(0.0,0.0,0.0);
            col = col + color(ray, world, 0);
            // accumulate color in the image data
            image.data[(j*image.width+i) as usize] = image.data[(j*image.width+i) as usize] + col;
        }
    }
    image.samples += 1;
}
