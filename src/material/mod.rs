use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: Color, scattered: Ray) -> bool;
}


pub struct Lambertian {
    albedo: Color,

}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian{albedo: albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord, mut attenuation: Color, mut scattered: Ray) -> bool {
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        scattered = Ray::new(rec.p, scatter_direction);
        attenuation = self.albedo;
        true
    }
}
