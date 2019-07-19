extern crate png;

mod pt_math;
use pt_math::Vec3;
use pt_math::Ray;
// use pt_math::*;

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

fn hit_sphere(center: Vec3, radius: f32, ray: Ray) -> bool {
    let oc = ray.origin - center;
    let a = pt_math::dot(ray.direction, ray.direction);
    let b = 2.0 * pt_math::dot(oc, ray.direction);
    let c = pt_math::dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, ray) {
        return Vec3::new(1.0,0.0,0.0);
    }
    let unit_dir = pt_math::unit_vector(&ray.direction);
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0,1.0,1.0)*(1.0-t) + Vec3::new(0.5,0.7,1.0)*t
}

fn main() {
    println!("hello world!");
    // image and camera data
    let width = 200;
    let height = 100;
    let mut image = Image::new(width, height);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    // fill image
    for j in 0..height {
        for i in 0..width {
            let u = (i as f32) / (width as f32);
            let v = (j as f32) / (height as f32);
            let ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
            let col = color(ray);
            image.data[(j*width+i) as usize] = col;
        }
    }
    // image.data[150] = Vec3::new(1.0,0.0,0.0);
    // image.data[340] = Vec3::new(1.0,1.0,0.0);
    image.save("ciccio.png");
}

