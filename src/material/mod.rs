use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

pub trait Scatter {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}


#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Color,

}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian{albedo: albedo}
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}
