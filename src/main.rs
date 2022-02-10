#![allow(non_snake_case)]
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Write};
use rand::Rng;
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

const ASPECT_RATIO: f64 = 4.0 / 3.0;
const IMAGE_WIDTH:  u32 = 400;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as u32;

fn write_image(filename: &str, w: u32, h: u32, buffer: &mut [Color])  {
    let mut buf = vec![0; buffer.len()*3];
    for i in 0..buffer.len()-3 {
        buf[i*3] =     (buffer[i].r()*255.0) as u8;
        buf[(i*3)+1] = (buffer[i].g()*255.0) as u8;
        buf[(i*3)+2] = (buffer[i].b()*255.0) as u8;
    }
    image::save_buffer(filename, &buf, w, h, image::ColorType::Rgb8).unwrap()
}

fn write_color(buffer: &mut [Color], x: u32, y: u32, color: Color, samples_per_pixel: u32) {
    let offset: usize = (x+((IMAGE_HEIGHT-1)-y)*IMAGE_WIDTH) as usize;
    let scale: f64    = 1.0 / samples_per_pixel as f64;
    buffer[offset]    = color*scale;
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

    let mut buffer = [Color{r: 0.0, g:0.0, b:0.0}; (IMAGE_WIDTH*IMAGE_HEIGHT) as usize];
    let samples_per_pixel  = 10;
    let mut rng = rand::thread_rng();


    let cam = Camera::new(ASPECT_RATIO);

    println!("Image {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let start_time = SystemTime::now();
    for y in (0..IMAGE_HEIGHT).rev() {
        print!("\rLine {}        ",y);
        io::stdout().flush().unwrap();
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH-1) as f64;
                let v = (y as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT-1) as f64;
                let r: Ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world);
            }
            write_color(&mut buffer, x, y, pixel_color, samples_per_pixel);
        }
    }
    println!("");
    let end_time = SystemTime::now();

    let s = start_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let e = end_time.duration_since(UNIX_EPOCH).expect("Time went backwards");

    println!("{:?}", e-s);

    write_image("test.png", IMAGE_WIDTH, IMAGE_HEIGHT, &mut buffer);
}
