use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin : Vec3,
    lower_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let viewport_height: f64 = 2.0;
        let viewport_width:  f64 = aspect_ratio * viewport_height;

        let theta = vfov.to_radians();
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2 - vertical/2 - w;

        Camera {origin: origin,
        horizontal: horizontal,
        vertical: vertical,
        lower_left_corner: lower_left_corner}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }



}

