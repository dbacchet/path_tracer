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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pt_math::dot;

    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn camera_creation() {
        let from = Vec3::new(0.0, 0.0, 0.0);
        let to = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vfov = 90.0;
        let aspect = 16.0 / 9.0;
        let aperture = 0.1;
        let focus_dist = 1.0;
        
        let camera = Camera::new(from, to, up, vfov, aspect, aperture, focus_dist);
        
        // Camera is created successfully
        assert_eq!(camera.origin.x, 0.0);
        assert_eq!(camera.origin.y, 0.0);
        assert_eq!(camera.origin.z, 0.0);
    }

    #[test]
    fn camera_ray_generation() {
        let from = Vec3::new(0.0, 0.0, 0.0);
        let to = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vfov = 90.0;
        let aspect = 2.0;
        let aperture = 0.0; // No depth of field for predictable test
        let focus_dist = 1.0;
        
        let camera = Camera::new(from, to, up, vfov, aspect, aperture, focus_dist);
        
        // Get ray through center of viewport
        let ray = camera.get_ray(0.5, 0.5);
        
        // Ray should originate from camera position
        assert_eq!(ray.origin.x, 0.0);
        assert_eq!(ray.origin.y, 0.0);
        assert_eq!(ray.origin.z, 0.0);
        
        // Ray direction should generally point in -z direction
        assert!(ray.direction.z < 0.0);
    }

    #[test]
    fn camera_ray_corners() {
        let from = Vec3::new(0.0, 0.0, 0.0);
        let to = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vfov = 90.0;
        let aspect = 1.0;
        let aperture = 0.0;
        let focus_dist = 1.0;
        
        let camera = Camera::new(from, to, up, vfov, aspect, aperture, focus_dist);
        
        // Get rays at different viewport positions
        let ray_bottom_left = camera.get_ray(0.0, 0.0);
        let ray_top_right = camera.get_ray(1.0, 1.0);
        let ray_center = camera.get_ray(0.5, 0.5);
        
        // All rays should point generally in -z direction
        assert!(ray_bottom_left.direction.z < 0.0);
        assert!(ray_top_right.direction.z < 0.0);
        assert!(ray_center.direction.z < 0.0);
        
        // Bottom left ray should point down and left
        assert!(ray_bottom_left.direction.x < ray_center.direction.x);
        assert!(ray_bottom_left.direction.y < ray_center.direction.y);
        
        // Top right ray should point up and right
        assert!(ray_top_right.direction.x > ray_center.direction.x);
        assert!(ray_top_right.direction.y > ray_center.direction.y);
    }

    #[test]
    fn camera_different_positions() {
        let from = Vec3::new(5.0, 2.0, 3.0);
        let to = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vfov = 45.0;
        let aspect = 16.0 / 9.0;
        let aperture = 0.1;
        let focus_dist = 5.0;
        
        let camera = Camera::new(from, to, up, vfov, aspect, aperture, focus_dist);
        
        // Camera origin should be at 'from' position
        assert_eq!(camera.origin.x, 5.0);
        assert_eq!(camera.origin.y, 2.0);
        assert_eq!(camera.origin.z, 3.0);
        
        // Get a ray
        let ray = camera.get_ray(0.5, 0.5);
        
        // Ray should have valid direction
        assert!(ray.direction.length() > 0.0);
    }

    #[test]
    fn camera_different_fov() {
        let from = Vec3::new(0.0, 0.0, 0.0);
        let to = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let aspect = 1.0;
        let aperture = 0.0;
        let focus_dist = 1.0;
        
        // Wide FOV camera
        let camera_wide = Camera::new(from, to, up, 120.0, aspect, aperture, focus_dist);
        
        // Narrow FOV camera
        let camera_narrow = Camera::new(from, to, up, 30.0, aspect, aperture, focus_dist);
        
        let ray_wide = camera_wide.get_ray(1.0, 0.5);
        let ray_narrow = camera_narrow.get_ray(1.0, 0.5);
        
        // Wide FOV should have rays spreading more
        // (comparing how much x varies for same u coordinate)
        let wide_angle = ray_wide.direction.x.abs() / ray_wide.direction.z.abs();
        let narrow_angle = ray_narrow.direction.x.abs() / ray_narrow.direction.z.abs();
        
        assert!(wide_angle > narrow_angle);
    }

    #[test]
    fn camera_lens_radius() {
        let from = Vec3::new(0.0, 0.0, 0.0);
        let to = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vfov = 90.0;
        let aspect = 1.0;
        let aperture = 2.0;
        let focus_dist = 1.0;
        
        let camera = Camera::new(from, to, up, vfov, aspect, aperture, focus_dist);
        
        // Lens radius should be half of aperture
        assert_eq!(camera.lens_radius, 1.0);
    }

    #[test]
    fn random_in_unit_disc_test() {
        // Test that random points are within unit disc and z=0
        for _ in 0..10 {
            let p = random_in_unit_disc();
            assert!(p.squared_length() <= 1.0);
            assert_eq!(p.z, 0.0); // Should be in xy-plane
        }
    }

    #[test]
    fn camera_orthogonal_basis() {
        let from = Vec3::new(1.0, 1.0, 1.0);
        let to = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vfov = 90.0;
        let aspect = 1.0;
        let aperture = 0.0;
        let focus_dist = 1.0;
        
        let camera = Camera::new(from, to, up, vfov, aspect, aperture, focus_dist);
        
        // u and v should be orthogonal
        let dot_uv = dot(camera.u, camera.v);
        assert!(approx_eq(dot_uv, 0.0, 0.0001));
        
        // Both should be unit vectors
        assert!(approx_eq(camera.u.length(), 1.0, 0.0001));
        assert!(approx_eq(camera.v.length(), 1.0, 0.0001));
    }
}
