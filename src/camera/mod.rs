use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin : Vec3,
    lower_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_disc: f64) -> Self {

        let theta = vfov.to_radians();
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width:  f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_disc * viewport_width * u;
        let vertical = focus_disc * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_disc*w;
        let lens_radius = aperture / 2.0;

        Camera {origin: origin,
        horizontal: horizontal,
        vertical: vertical,
        lower_left_corner: lower_left_corner,
        u: u, v: v, w: w,
        lens_radius: lens_radius}
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {

        let rd1: Vec3 = Vec3::random_in_unit_disk();
        let rd: Vec3 = rd1 * self.lens_radius;
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray::new(self.origin+offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset)
    }



}

