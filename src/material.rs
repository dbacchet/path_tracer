use crate::pt_math::{Vec3, Ray, dot, unit_vector};

use rand::Rng;

pub struct Scatter {
    pub ray: Ray,
    pub color: Vec3,
}

impl Scatter {
    pub fn new(ray: Ray, color: Vec3) -> Scatter {
        Scatter { ray, color }
    }
}

pub trait Material {
    // given an input ray, hit point and normal, calculate the scattered output ray and its attenuation
    fn scatter(&self, ray_in: Ray, hit_point: Vec3, hit_normal: Vec3) -> Option<Scatter>;
}

// /////////////////// //
// Lambertian material //
// /////////////////// //
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit_point: Vec3, hit_normal: Vec3) -> Option<Scatter> {
        let target = hit_point + hit_normal + random_in_unit_sphere();
        return Some( Scatter::new( Ray::new(hit_point, target-hit_point), self.albedo ) );
    }
}

// ////////////// //
// Metal material //
// ////////////// //
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }

}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit_point: Vec3, hit_normal: Vec3) -> Option<Scatter> {
        let reflected_dir = reflect(unit_vector(ray_in.direction), hit_normal) + self.fuzz*random_in_unit_sphere();
        if dot(reflected_dir, hit_normal)>0.0 {
            return Some( Scatter::new( Ray::new(hit_point, reflected_dir), self.albedo ));
        }
        return None;
    }
}

// /////////////////// //
// Dielectric material //
// /////////////////// //
pub struct Dielectric {
    refraction_index: f32,
    fuzz: f32,
    attenuation: Vec3,
}

impl Dielectric {
    pub fn new(refraction_index: f32, fuzz: f32) -> Dielectric {
        Dielectric { refraction_index, fuzz, attenuation: Vec3::new(1.0,1.0,1.0) }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit_point: Vec3, hit_normal: Vec3) -> Option<Scatter> {
        let dn_dot = dot(ray_in.direction, hit_normal);
        let ray_dir_len = ray_in.direction.length();
        let mut outward_normal = -hit_normal;
        let mut ni_over_nt = self.refraction_index;
        let mut cosine = self.refraction_index * dn_dot / ray_dir_len;
        // swap normal and data if is pointing inside
        if dn_dot<=0.0 {
            outward_normal = hit_normal;
            ni_over_nt = 1.0/self.refraction_index;
            cosine = -dn_dot / ray_dir_len;
        }
        // calc reflected/refracted ray
        let reflected_dir = reflect(unit_vector(ray_in.direction), hit_normal) + self.fuzz*random_in_unit_sphere();
        if let Some(refracted_dir) = refract(ray_in.direction, outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refraction_index);
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < reflect_prob {
                return Some( Scatter::new( Ray::new(hit_point, reflected_dir), self.attenuation) );
            } else {
                let refracted_dir = refracted_dir + self.fuzz*random_in_unit_sphere();
                return Some( Scatter::new( Ray::new(hit_point, refracted_dir), self.attenuation) );
            }
        }
        return Some( Scatter::new( Ray::new(hit_point, reflected_dir), self.attenuation) );
    }
}



fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0*dot(v, n)*n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let u = unit_vector(v);
    let dt = dot(u, n);
    let discriminant = 1.0- ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if discriminant>0.0 {
        let refracted = ni_over_nt * (u - n*dt) - n*discriminant.sqrt();
        return Some(refracted);
    }
    return None;
}

// Schlick's approximation for refelctivity function of angle
fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0-refraction_index) / (1.0+refraction_index);
    let r0 = r0*r0;
    r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>());
        let p = p*2.0 - Vec3::new(1.0,1.0,1.0); // values are in the range [-1,1) now
        if p.squared_length()<=1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_reflect() {
        // Reflect a vector off a horizontal surface (normal pointing up)
        let v = Vec3::new(1.0, -1.0, 0.0); // 45 degree angle down
        let n = Vec3::new(0.0, 1.0, 0.0);  // Normal pointing up
        let reflected = reflect(v, n);
        
        assert_eq!(reflected.x, 1.0);
        assert_eq!(reflected.y, 1.0); // Should bounce up
        assert_eq!(reflected.z, 0.0);
    }

    #[test]
    fn test_reflect_perpendicular() {
        // Perpendicular reflection
        let v = Vec3::new(0.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let reflected = reflect(v, n);
        
        assert_eq!(reflected.x, 0.0);
        assert_eq!(reflected.y, 1.0); // Should reflect straight back up
        assert_eq!(reflected.z, 0.0);
    }

    #[test]
    fn test_refract_basic() {
        // Test basic refraction
        let v = Vec3::new(0.0, -1.0, 0.0);  // Straight down
        let n = Vec3::new(0.0, 1.0, 0.0);   // Normal pointing up
        let ni_over_nt = 1.0 / 1.5;         // Air to glass
        
        let refracted = refract(v, n, ni_over_nt);
        assert!(refracted.is_some());
        
        let r = refracted.unwrap();
        // Should bend towards the normal when entering denser medium
        assert!(r.y < 0.0); // Still going down
    }

    #[test]
    fn test_refract_total_internal_reflection() {
        // Test total internal reflection (should return None)
        let v = Vec3::new(0.9, -0.1, 0.0);  // Grazing angle
        let n = Vec3::new(0.0, 1.0, 0.0);
        let ni_over_nt = 1.5;  // Glass to air (critical angle case)
        
        let refracted = refract(v, n, ni_over_nt);
        assert!(refracted.is_none());
    }

    #[test]
    fn test_refract_perpendicular() {
        // Perpendicular ray should pass through without bending
        let v = Vec3::new(0.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let ni_over_nt = 1.0;  // Same refractive index
        
        let refracted = refract(v, n, ni_over_nt);
        assert!(refracted.is_some());
        
        let r = refracted.unwrap();
        assert!(approx_eq(r.x, 0.0, 0.001));
        assert!(r.y < 0.0);
        assert!(approx_eq(r.z, 0.0, 0.001));
    }

    #[test]
    fn test_schlick() {
        // Test Schlick's approximation
        let cosine = 0.0; // 90 degrees
        let ref_idx = 1.5;
        let reflectance = schlick(cosine, ref_idx);
        
        // At grazing angle, reflectance should be high
        assert!(reflectance > 0.0 && reflectance <= 1.0);
        
        // At perpendicular incidence
        let cosine = 1.0;
        let reflectance = schlick(cosine, ref_idx);
        
        // Should be lower at perpendicular
        assert!(reflectance > 0.0 && reflectance < 0.1);
    }

    #[test]
    fn test_schlick_edge_cases() {
        // Test at different angles
        let ref_idx = 1.5;
        
        let r0 = schlick(0.0, ref_idx);
        let r1 = schlick(1.0, ref_idx);
        
        // Reflectance increases as angle increases
        assert!(r0 >= r1);
    }

    #[test]
    fn scatter_creation() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let color = Vec3::new(0.8, 0.3, 0.3);
        let scatter = Scatter::new(ray, color);
        
        assert_eq!(scatter.ray.origin.x, 0.0);
        assert_eq!(scatter.color.x, 0.8);
        assert_eq!(scatter.color.y, 0.3);
        assert_eq!(scatter.color.z, 0.3);
    }

    #[test]
    fn lambertian_creation() {
        let albedo = Vec3::new(0.8, 0.3, 0.3);
        let lambertian = Lambertian::new(albedo);
        
        assert_eq!(lambertian.albedo.x, 0.8);
        assert_eq!(lambertian.albedo.y, 0.3);
        assert_eq!(lambertian.albedo.z, 0.3);
    }

    #[test]
    fn lambertian_scatter() {
        let albedo = Vec3::new(0.8, 0.3, 0.3);
        let lambertian = Lambertian::new(albedo);
        
        let ray_in = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit_point = Vec3::new(0.0, 0.0, 0.0);
        let hit_normal = Vec3::new(0.0, 1.0, 0.0);
        
        let scatter = lambertian.scatter(ray_in, hit_point, hit_normal);
        assert!(scatter.is_some());
        
        let s = scatter.unwrap();
        assert_eq!(s.color.x, 0.8);
        assert_eq!(s.color.y, 0.3);
        assert_eq!(s.color.z, 0.3);
    }

    #[test]
    fn metal_creation() {
        let albedo = Vec3::new(0.8, 0.6, 0.2);
        let fuzz = 0.1;
        let metal = Metal::new(albedo, fuzz);
        
        assert_eq!(metal.albedo.x, 0.8);
        assert_eq!(metal.fuzz, 0.1);
    }

    #[test]
    fn metal_scatter_reflection() {
        let albedo = Vec3::new(0.8, 0.6, 0.2);
        let metal = Metal::new(albedo, 0.0); // No fuzz for predictable test
        
        let ray_in = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit_point = Vec3::new(0.0, 0.0, 0.0);
        let hit_normal = Vec3::new(0.0, 1.0, 0.0);
        
        let scatter = metal.scatter(ray_in, hit_point, hit_normal);
        assert!(scatter.is_some());
        
        let s = scatter.unwrap();
        // Reflected ray should point upward
        assert!(s.ray.direction.y > 0.0);
    }

    #[test]
    fn metal_scatter_grazing_angle() {
        let albedo = Vec3::new(0.8, 0.6, 0.2);
        let metal = Metal::new(albedo, 0.0);
        
        // Ray at grazing angle (almost parallel to surface)
        let ray_in = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, -0.01, 0.0));
        let hit_point = Vec3::new(0.0, 0.0, 0.0);
        let hit_normal = Vec3::new(0.0, 1.0, 0.0);
        
        let scatter = metal.scatter(ray_in, hit_point, hit_normal);
        // May or may not scatter depending on exact angle
        // Just checking it doesn't panic
        assert!(scatter.is_some() || scatter.is_none());
    }

    #[test]
    fn dielectric_creation() {
        let ref_idx = 1.5;
        let fuzz = 0.0;
        let dielectric = Dielectric::new(ref_idx, fuzz);
        
        assert_eq!(dielectric.refraction_index, 1.5);
        assert_eq!(dielectric.fuzz, 0.0);
        assert_eq!(dielectric.attenuation.x, 1.0);
    }

    #[test]
    fn dielectric_scatter() {
        let dielectric = Dielectric::new(1.5, 0.0);
        
        let ray_in = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit_point = Vec3::new(0.0, 0.0, 0.0);
        let hit_normal = Vec3::new(0.0, 1.0, 0.0);
        
        let scatter = dielectric.scatter(ray_in, hit_point, hit_normal);
        assert!(scatter.is_some());
        
        let s = scatter.unwrap();
        // Should have white attenuation for glass
        assert_eq!(s.color.x, 1.0);
        assert_eq!(s.color.y, 1.0);
        assert_eq!(s.color.z, 1.0);
    }

    #[test]
    fn random_in_unit_sphere_test() {
        // Test that random points are within unit sphere
        for _ in 0..10 {
            let p = random_in_unit_sphere();
            assert!(p.squared_length() <= 1.0);
        }
    }
}
