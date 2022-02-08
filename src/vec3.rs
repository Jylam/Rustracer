use std::ops::Add;
use std::ops::Sub;
use std::fmt::{self, Formatter, Display};

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3{x: x, y: y, z: z}
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
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

