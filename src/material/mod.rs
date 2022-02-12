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



pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: f
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).unit();
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere());

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
