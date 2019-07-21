use crate::pt_math::Vec3;
use crate::pt_math::Ray;
use crate::pt_math;
use crate::material::Material;


pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material
}

// hitable trait
pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}


// a sphere, defined with center and radisu
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {center, radius, material}
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = pt_math::dot(ray.direction, ray.direction);
        let b = pt_math::dot(oc, ray.direction);
        let c = pt_math::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - a*c; // removed the 2.0 factor from b and the 4.0 here, since the result is the same 
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt())/a;
            if t<t_max && t>t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point-self.center) / self.radius;
                return Some(HitRecord{t, point, normal, material: &*self.material});
            }
        } else {
            let t = (-b + discriminant.sqrt())/a;
            if t<t_max && t>t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point-self.center) / self.radius;
                return Some(HitRecord{t, point, normal, material: &*self.material});
            }
        }
        return None;
    }
}

// list of objects that implement the Hitable trait

pub struct HitableList {
    objects: Vec<Sphere> // TODO: use a trait object to make it generic. for now only consider spheres
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest = t_max;
        for obj in &self.objects {
            if let Some(hr) = obj.hit(ray, t_min, closest) {
                closest = hr.t;
                hit_record = Some(hr);
            }
        }
        return hit_record;
    }
}
