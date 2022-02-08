use std::ops::Add;
use std::fmt::{self, Formatter, Display};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

