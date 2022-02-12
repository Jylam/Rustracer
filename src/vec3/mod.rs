#![allow(dead_code)]
use rand::Rng;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;
use std::fmt::{self, Formatter, Display};
use crate::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3{x: x, y: y, z: z}
    }

    // Accessors
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    // Operations
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn dot(&self, v2: Vec3) -> f64 {
        self.x*v2.x + self.y*v2.y + self.z*v2.z
    }
    pub fn cross(&self, v2: Vec3) -> Vec3 {
        Vec3{x: self.y*v2.z - self.z*v2.y,
             y: self.z*v2.x - self.x*v2.z,
             z: self.x*v2.y - self.y*v2.x}
    }
    pub fn unit(&self) -> Vec3 {
        *self/self.length()
    }
    pub fn random_mm(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p: Vec3;
        loop {
            p = Vec3::random_mm(-1.0,1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Vec3{
            Vec3::random_in_unit_sphere().unit()
    }
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere: Vec3 = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }
    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3{
        let cos_theta: f64 =  f64::min(-self.dot(n), 1.0);
        let r_out_perp: Vec3 =  etai_over_etat * (self + cos_theta*n);
        let r_out_parallel: Vec3  = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
        r_out_perp + r_out_parallel
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v2: Vec3) -> Vec3 {
        Vec3 {x: self.x + v2.x, y: self.y + v2.y, z: self.z + v2.z}
    }
}
impl Add<Color> for Vec3 {
    type Output = Color;
    fn add(self, v2: Color) -> Color {
        Color {r: self.x + v2.r, g: self.y + v2.g, b: self.z + v2.b}
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v2: Vec3) -> Vec3 {
        Vec3 {x: self.x - v2.x, y: self.y - v2.y, z: self.z - v2.z}
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v2: Vec3) -> Vec3 {
        Vec3 {x: self.x * v2.x, y: self.y * v2.y, z: self.z * v2.z}
    }
}
impl Mul<i32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: i32) -> Vec3 {
        Vec3 {x: self.x * t as f64, y: self.y * t as f64, z: self.z * t as f64}
    }
}
impl Mul<Vec3> for i32 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        Vec3 {x: t.x * self as f64, y: t.y * self as f64, z: t.z * self as f64}
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {x: self.x * t, y: self.y * t, z: self.z * t}
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        Vec3 {x: t.x * self as f64, y: t.y * self as f64, z: t.z * self as f64}
    }
}
impl Div<i32> for Vec3 {
    type Output = Vec3;
    fn div(self, t: i32) -> Vec3 {
        Vec3 {x: self.x / t as f64, y: self.y / t as f64, z: self.z / t as f64}
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3 {x: self.x / t, y: self.y / t, z: self.z / t}
    }
}


