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
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
        ()
    }
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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, mut rec: HitRecord) -> bool {

        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = oc.dot(r.direction());
        let c: f64 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f64 = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);


        rec.normal = (rec.p - self.center) / self.radius;

        true
    }




}

