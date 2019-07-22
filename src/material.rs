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
