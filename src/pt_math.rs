use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone,Copy,Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn squared_length(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3{
        Vec3 { x: self.x + rhs.x,
               y: self.y + rhs.y,
               z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3{
        Vec3 { x: self.x - rhs.x,
               y: self.y - rhs.y,
               z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x * rhs,
               y: self.y * rhs,
               z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: rhs.x * self,
               y: rhs.y * self,
               z: rhs.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x / rhs,
               y: self.y / rhs,
               z: self.z / rhs,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: rhs.x / self,
               y: rhs.y / self,
               z: rhs.z / self,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x,
               y: -self.y,
               z: -self.z,
        }
    }
}

// finctions working on Vec3 

pub fn unit_vector(v: Vec3) -> Vec3 {
    let len = v.length();
    Vec3 {x: v.x/len,
          y: v.y/len,
          z: v.z/len
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 { x: v1.y*v2.z - v1.z*v2.y,
           y: v1.z*v2.x - v1.x*v2.z,
           z: v1.x*v2.y - v1.y*v2.x,
    }
}

pub fn mul_component(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 { x: v1.x*v2.x,
           y: v1.y*v2.y,
           z: v1.z*v2.z,
    }
}

// Ray in 3D space
#[derive(Clone,Copy,Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }
    
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {

    use super::Vec3;
    use super::Ray;

    #[test]
    fn vec3_creation() {
        let v1 = super::Vec3::new(1.0,2.0,3.0);
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);

        let v2 = v1 + super::Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 3.0);
        assert_eq!(v2.z, 4.0);

        let v3 = v1 * 2.0;
        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 6.0);
    }

    use super::unit_vector;
    use super::cross;

    #[test]
    fn vec3_functions() {
        let v1 = Vec3::new(3.0,0.0,0.0);
        let v2 = unit_vector(v1);
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
        let v1 = Vec3::new(1.0,0.0,0.0);
        let v2 = Vec3::new(0.0,1.0,0.0);
        let v3 = cross(v1, v2);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, 1.0);
        let v3 = cross(v2, v1);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, -1.0);

    }

    #[test]
    fn ray() {
        let r = Ray::new(Vec3::new(1.0,2.0,3.0), Vec3::new(1.0,1.0,1.0));
        let p = r.point_at_parameter(1.0);
        assert_eq!(p.x, 2.0);
        assert_eq!(p.y, 3.0);
        assert_eq!(p.z, 4.0);
        let p = r.point_at_parameter(1.5);
        assert_eq!(p.x, 2.5);
        assert_eq!(p.y, 3.5);
        assert_eq!(p.z, 4.5);
    }
}

