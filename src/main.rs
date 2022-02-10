#![allow(non_snake_case)]
use std::time::{SystemTime, UNIX_EPOCH};
extern crate image;

mod vec3;
use crate::vec3::Vec3;

mod color;
use crate::color::Color;

mod ray;
use crate::ray::Ray;

mod hittable;
use crate::hittable::Hittable;
use crate::hittable::Sphere;
use crate::hittable::World;

mod camera;
use crate::camera::Camera;

fn write_image(filename: &str, w: u32, h: u32, buffer: &[Color])  {
    let mut buf = vec![0; buffer.len()*3];
    for i in 0..buffer.len()-3 {
        buf[i*3] =     (buffer[i].r()*255.0) as u8;
        buf[(i*3)+1] = (buffer[i].g()*255.0) as u8;
        buf[(i*3)+2] = (buffer[i].b()*255.0) as u8;
    }
    image::save_buffer(filename, &buf, w, h, image::ColorType::Rgb8).unwrap()
}

fn ray_color(r: Ray, world: &World) -> Color {

    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        (rec.normal + Color::new(1.0, 1.0, 1.0))*0.5
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0)* (1.0 - t) +  Color::new(0.5, 0.7, 1.0) * t
    }

}
fn main() {

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:  u32 = 400;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as u32;

    let mut buffer = [Color{r: 0.0, g:0.0, b:0.0}; (IMAGE_WIDTH*IMAGE_HEIGHT) as usize];

    let viewport_height: f64 = 2.0;
    let viewport_width:  f64 = ASPECT_RATIO * viewport_height;
    let focal_length:    f64 = 1.0;

    let origin:     Vec3 = Vec3::new(0.0,            0.0,               0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0,               0.0);
    let vertical:   Vec3 = Vec3::new(0.0,            viewport_height,   0.0);
    let lower_left_corner = origin - horizontal/2 - vertical/2 - Vec3::new(0.0, 0.0, focal_length);


    println!("Image {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let start_time = SystemTime::now();
    for y in (0..IMAGE_HEIGHT).rev() {
        print!("\rLine {}",y);
        for x in 0..IMAGE_WIDTH {
            let u: f64 = x as f64 / (IMAGE_WIDTH-1) as f64;
            let v: f64 = y as f64 / (IMAGE_HEIGHT-1) as f64;

            let r: Ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let pixel_color: Color = ray_color(r, &world);

            let offset: usize = (x+((IMAGE_HEIGHT-1)-y)*IMAGE_WIDTH) as usize;
            buffer[offset] = pixel_color;
        }
    }
    println!("");
    let end_time = SystemTime::now();

    let s = start_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let e = end_time.duration_since(UNIX_EPOCH).expect("Time went backwards");

    println!("{:?}", e-s);

    write_image("test.png", IMAGE_WIDTH, IMAGE_HEIGHT, &buffer);
}
