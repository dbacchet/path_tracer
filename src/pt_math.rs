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
    use super::*;

    // Helper function for floating point comparison
    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn vec3_creation() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
    }

    #[test]
    fn vec3_addition() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, 4.0);
    }

    #[test]
    fn vec3_subtraction() {
        let v1 = Vec3::new(5.0, 7.0, 9.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 5.0);
    }

    #[test]
    fn vec3_scalar_multiplication() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        
        // Test Vec3 * f32
        let v2 = v1 * 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 6.0);
        
        // Test f32 * Vec3
        let v3 = 3.0 * v1;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn vec3_scalar_division() {
        let v1 = Vec3::new(6.0, 8.0, 10.0);
        
        // Test Vec3 / f32
        let v2 = v1 / 2.0;
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 5.0);
        
        // Test f32 / Vec3 (divides each component by the scalar)
        let v3 = Vec3::new(6.0, 8.0, 10.0);
        let v4 = 2.0 / v3;
        assert_eq!(v4.x, 3.0);
        assert_eq!(v4.y, 4.0);
        assert_eq!(v4.z, 5.0);
    }

    #[test]
    fn vec3_negation() {
        let v1 = Vec3::new(1.0, -2.0, 3.0);
        let v2 = -v1;
        assert_eq!(v2.x, -1.0);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, -3.0);
    }

    #[test]
    fn vec3_length() {
        let v1 = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v1.length(), 5.0);
        
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert!(approx_eq(v2.length(), 1.732050807, 0.0001));
    }

    #[test]
    fn vec3_squared_length() {
        let v1 = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v1.squared_length(), 25.0);
        
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v2.squared_length(), 14.0);
    }

    #[test]
    fn unit_vector_test() {
        let v1 = Vec3::new(3.0, 0.0, 0.0);
        let v2 = unit_vector(v1);
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
        assert!(approx_eq(v2.length(), 1.0, 0.0001));
        
        let v3 = Vec3::new(1.0, 2.0, 2.0);
        let v4 = unit_vector(v3);
        assert!(approx_eq(v4.length(), 1.0, 0.0001));
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = dot(v1, v2);
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(result, 32.0);
        
        // Test perpendicular vectors (dot product should be 0)
        let v3 = Vec3::new(1.0, 0.0, 0.0);
        let v4 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(dot(v3, v4), 0.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        
        // Cross product of x and y should give z
        let v3 = cross(v1, v2);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, 1.0);
        
        // Cross product in opposite order should give -z
        let v4 = cross(v2, v1);
        assert_eq!(v4.x, 0.0);
        assert_eq!(v4.y, 0.0);
        assert_eq!(v4.z, -1.0);
        
        // Test with non-unit vectors
        let v5 = Vec3::new(2.0, 3.0, 4.0);
        let v6 = Vec3::new(5.0, 6.0, 7.0);
        let v7 = cross(v5, v6);
        // (3*7 - 4*6, 4*5 - 2*7, 2*6 - 3*5) = (21-24, 20-14, 12-15) = (-3, 6, -3)
        assert_eq!(v7.x, -3.0);
        assert_eq!(v7.y, 6.0);
        assert_eq!(v7.z, -3.0);
    }

    #[test]
    fn mul_component_test() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(5.0, 6.0, 7.0);
        let v3 = mul_component(v1, v2);
        assert_eq!(v3.x, 10.0);
        assert_eq!(v3.y, 18.0);
        assert_eq!(v3.z, 28.0);
    }

    #[test]
    fn ray_creation() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(1.0, 0.0, 0.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin.x, 1.0);
        assert_eq!(r.origin.y, 2.0);
        assert_eq!(r.origin.z, 3.0);
        assert_eq!(r.direction.x, 1.0);
        assert_eq!(r.direction.y, 0.0);
        assert_eq!(r.direction.z, 0.0);
    }

    #[test]
    fn ray_point_at_parameter() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 1.0, 1.0));
        
        let p0 = r.point_at_parameter(0.0);
        assert_eq!(p0.x, 1.0);
        assert_eq!(p0.y, 2.0);
        assert_eq!(p0.z, 3.0);
        
        let p1 = r.point_at_parameter(1.0);
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 3.0);
        assert_eq!(p1.z, 4.0);
        
        let p2 = r.point_at_parameter(1.5);
        assert_eq!(p2.x, 2.5);
        assert_eq!(p2.y, 3.5);
        assert_eq!(p2.z, 4.5);
        
        let p3 = r.point_at_parameter(-1.0);
        assert_eq!(p3.x, 0.0);
        assert_eq!(p3.y, 1.0);
        assert_eq!(p3.z, 2.0);
    }

    #[test]
    fn vec3_zero() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v.length(), 0.0);
        assert_eq!(v.squared_length(), 0.0);
    }

    #[test]
    fn combined_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        
        // Test (v1 + v2) * 2.0
        let result = (v1 + v2) * 2.0;
        assert_eq!(result.x, 10.0);
        assert_eq!(result.y, 14.0);
        assert_eq!(result.z, 18.0);
        
        // Test negation with addition
        let v3 = -v1 + v2;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, 3.0);
    }
}

