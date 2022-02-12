use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
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
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
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
    pub fn new(a: Color, f: f64) -> Self {
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


pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric{ir: index_of_refraction}
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;
        r0 + (1.0-r0)*f64::powf(1.0 - cosine, 5.0)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };

        let unit_direction = r_in.direction().unit();
        let cos_theta = f64::min(-unit_direction.dot(rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > fastrand::f64() {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        }
        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }

}

