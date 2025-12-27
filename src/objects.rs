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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
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
    pub objects: Vec<Sphere> // TODO: use a trait object to make it generic. for now only consider spheres
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;

    #[test]
    fn sphere_creation() {
        let center = Vec3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(center, radius, material);
        
        // Sphere is created successfully (compilation confirms this works)
        assert_eq!(sphere.center.x, 0.0);
        assert_eq!(sphere.center.y, 0.0);
        assert_eq!(sphere.center.z, -1.0);
        assert_eq!(sphere.radius, 0.5);
    }

    #[test]
    fn sphere_hit_simple() {
        let center = Vec3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(center, radius, material);
        
        // Ray pointing directly at the sphere center
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = sphere.hit(&ray, 0.0, 100.0);
        
        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        assert!(hit_record.t > 0.0);
        assert!(hit_record.t < 1.0); // Should hit before reaching center
    }

    #[test]
    fn sphere_miss() {
        let center = Vec3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(center, radius, material);
        
        // Ray pointing away from the sphere
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let hit = sphere.hit(&ray, 0.0, 100.0);
        
        assert!(hit.is_none());
    }

    #[test]
    fn sphere_hit_normal() {
        let center = Vec3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(center, radius, material);
        
        // Ray from above hitting the top of the sphere
        let ray = Ray::new(Vec3::new(0.0, 1.0, -1.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = sphere.hit(&ray, 0.0, 100.0);
        
        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        // Normal at the top should point upward
        assert!(hit_record.normal.y > 0.9); // Should be close to 1.0
        assert_eq!(hit_record.normal.length(), 1.0);
    }

    #[test]
    fn sphere_hit_outside_range() {
        let center = Vec3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(center, radius, material);
        
        // Ray that would hit but t_max is too small
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = sphere.hit(&ray, 0.0, 0.1);
        
        assert!(hit.is_none());
    }

    #[test]
    fn hitable_list_creation() {
        let list = HitableList::new();
        assert_eq!(list.objects.len(), 0);
    }

    #[test]
    fn hitable_list_add() {
        let mut list = HitableList::new();
        let sphere1 = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
        );
        let sphere2 = Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
        );
        
        list.add(sphere1);
        list.add(sphere2);
        
        assert_eq!(list.objects.len(), 2);
    }

    #[test]
    fn hitable_list_hit_closest() {
        let mut list = HitableList::new();
        
        // Add two spheres, one closer than the other
        let sphere1 = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.3,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
        );
        let sphere2 = Sphere::new(
            Vec3::new(0.0, 0.0, -2.0),
            0.3,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
        );
        
        list.add(sphere1);
        list.add(sphere2);
        
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = list.hit(&ray, 0.0, 100.0);
        
        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        // Should hit the closer sphere (at z=-1)
        assert!(hit_record.t < 1.0);
    }

    #[test]
    fn hitable_list_no_hit() {
        let mut list = HitableList::new();
        
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.3,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
        );
        
        list.add(sphere);
        
        // Ray pointing away
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let hit = list.hit(&ray, 0.0, 100.0);
        
        assert!(hit.is_none());
    }
}
