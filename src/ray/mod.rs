use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Vec3,
    dir:  Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray{orig: orig, dir: dir}
    }

    // Accessors
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // Operations
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + (self.dir*t)
    }

}
