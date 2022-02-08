use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;
use std::fmt::{self, Formatter, Display};

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
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {x: self.x * t, y: self.y * t, z: self.z * t}
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


