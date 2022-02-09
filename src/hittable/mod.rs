use crate::vec3::Vec3;
use crate::color::Color;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}


#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    fn new(c: Vec3, r: f64) -> Self {
        Sphere{center: c, radius: r}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool {
        true
    }
}

