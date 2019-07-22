use crate::pt_math::{Vec3, Ray, unit_vector, cross};

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(from: Vec3, to: Vec3, up: Vec3, vfov_deg: f32, aspect_ratio: f32, aperture: f32, dist_to_focus: f32) -> Camera {
        let theta = vfov_deg*std::f32::consts::PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = unit_vector(from - to);
        let u = unit_vector(cross(up, w));
        let v = cross(w, u);
        Camera {
            lower_left_corner: from - half_width*dist_to_focus*u - half_height*dist_to_focus*v - dist_to_focus*w,
            horizontal: 2.0*half_width*dist_to_focus*u,
            vertical: 2.0*half_height*dist_to_focus*v,
            origin: from,
            u,
            v, 
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius*random_in_unit_disc();
        let offset = self.u*rd.x + self.v*rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset)
    }
}



use rand::Rng;

fn random_in_unit_disc() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0*Vec3::new(rng.gen::<f32>(),rng.gen::<f32>(),0.0) - Vec3::new(1.0,1.0,0.0);
        if p.squared_length()<=1.0 {
            return p;
        }
    }
}
