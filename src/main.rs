#![allow(non_snake_case)]
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Write};
use std::rc::Rc;
use rand::Rng;
extern crate term_size;
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

mod material;
use crate::material::{Lambertian, Metal};

mod camera;
use crate::camera::Camera;

const ASPECT_RATIO: f64 = 4.0 / 4.0;
const IMAGE_WIDTH:  u32 = 800;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 10;
const SAMPLES_PER_PIXEL: u32  = 1000;
const SCALE: f64    = 1.0 / (SAMPLES_PER_PIXEL as f64);

fn write_image(filename: &str, w: u32, h: u32, buffer: &mut [Color])  {
    let mut buf = vec![0; buffer.len()*3];
    for i in 0..buffer.len()-3 {
        buf[i*3] =     (f64::clamp(buffer[i].r(), 0.0, 0.999)*255.0) as u8;
        buf[(i*3)+1] = (f64::clamp(buffer[i].g(), 0.0, 0.999)*255.0) as u8;
        buf[(i*3)+2] = (f64::clamp(buffer[i].b(), 0.0, 0.999)*255.0) as u8;
    }
    image::save_buffer(filename, &buf, w, h, image::ColorType::Rgb8).unwrap()
}

fn write_color(buffer: &mut [Color], x: u32, y: u32, color: Color) {
    let offset: usize = (x+((IMAGE_HEIGHT-1)-y)*IMAGE_WIDTH) as usize;

    let r = f64::sqrt(SCALE * color.r);
    let g = f64::sqrt(SCALE * color.g);
    let b = f64::sqrt(SCALE * color.b);

    buffer[offset]    = Color::new(r, g, b);
}


fn ray_color(r: Ray, world: &World, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0,0.0,0.0);
    }

    if let Some(rec) = world.hit(r, 0.01, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0)* (1.0 - t) +  Color::new(0.5, 0.7, 1.0) * t
    }

}

fn print_progress(width: usize, progress: f64) {

    let count = width as f64 * progress;
    print!("\r");
    for _x in 0..count as u32 {
        print!("❯");
        io::stdout().flush().unwrap();
    }
}

fn main() {

    let mut buffer: Vec<Color> = vec![Color{r: 0.0, g:0.0, b:0.0}; (IMAGE_WIDTH*IMAGE_HEIGHT) as usize];
    let mut rng = rand::thread_rng();


    let cam = Camera::new(ASPECT_RATIO);

    println!("Image {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mat1 = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.2)));
    let mat2 = Rc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let mat3 = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));

    let mut world = World::new();

    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat1)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat2)));
    world.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat3)));


    let term_w: usize;
    if let Some((w, _h)) = term_size::dimensions() {
        term_w = w;
    } else {
        term_w = 10;
    }

    let start_time = SystemTime::now();
    for y in (0..IMAGE_HEIGHT).rev() {
        print_progress(term_w, 1.0 - (((y+1) as f64 / IMAGE_HEIGHT as f64)));

        for x in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH-1) as f64;
                let v = (y as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT-1) as f64;
                let r: Ray = cam.get_ray(u, v);
                let color = ray_color(r, &world, MAX_DEPTH);
                pixel_color = pixel_color + color;
            }
            write_color(&mut buffer, x, y, pixel_color);
        }
    }
    println!("");
    let end_time = SystemTime::now();

    let s = start_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let e = end_time.duration_since(UNIX_EPOCH).expect("Time went backwards");

    println!("{:?}", e-s);

    write_image("test.png", IMAGE_WIDTH, IMAGE_HEIGHT, &mut buffer);
}
