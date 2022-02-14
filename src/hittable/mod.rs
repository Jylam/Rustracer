use std::boxed::Box;
use std::sync::Arc;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Scatter;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Scatter>,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal} else {-outward_normal};
    }
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;
        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}


#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(c: Vec3, r: f64, mat: Arc<dyn Scatter>) -> Self {
        Sphere{center: c, radius: r, mat: mat}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = oc.dot(r.direction());
        let c: f64 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f64 = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            mat: self.mat.clone(),
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}

