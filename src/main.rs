#![allow(non_snake_case)]
use std::time::Instant;
use std::io::{self, Write};
use std::sync::Arc;
use threadpool::ThreadPool;
use std::sync::mpsc;
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
use crate::material::{Lambertian, Metal, Dielectric};

mod camera;
use crate::camera::Camera;

const ASPECT_RATIO: f64 = 4.0 / 3.0;
const IMAGE_WIDTH:  u32 = 800;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32  = 10;
const SCALE: f64    = 1.0 / (SAMPLES_PER_PIXEL as f64);

fn write_image(filename: &str, w: u32, h: u32, buffer: &mut [Color])  {
    let mut buf = vec![0; buffer.len()*3];
    for i in 0..buffer.len()-3 {
        buf[i*3] =     (f64::clamp(buffer[i].r(), 0.0, 0.999)*255.0) as u8;
        buf[(i*3)+1] = (f64::clamp(buffer[i].g(), 0.0, 0.999)*255.0) as u8;
        buf[(i*3)+2] = (f64::clamp(buffer[i].b(), 0.0, 0.999)*255.0) as u8;
    }
    image::save_buffer(filename, &buf, w, h, image::ColorType::Rgb8).unwrap();
    println!("Saved {}", filename);
}

fn put_pixel(buffer: &mut [Color], x: u32, y: u32, color: Color) {
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

fn create_world(seed: u64) -> World {

    fastrand::seed(seed);
    let mat_lambert = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mat_metal = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    let mat_glass = Arc::new(Dielectric::new(1.5));

    let mut world = World::new();

    world.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, -0.0), 1000.0, mat_ground)));

    world.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat_lambert)));
    world.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat_metal)));

    world.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat_glass.clone())));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), -0.95, mat_glass.clone())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = fastrand::f64();
            let center: Vec3 = Vec3::new(a as f64 + 0.9*fastrand::f64(), 0.2, b as f64 + 0.9*fastrand::f64());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    let sphere_material = Arc::new(Lambertian::new(Color::new(fastrand::f64(), fastrand::f64(), fastrand::f64())));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let sphere_material = Arc::new(Metal::new(Color::new(fastrand::f64(), fastrand::f64(), fastrand::f64()), 0.0));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    world
}


fn compute_pixel(x: u32, y: u32, cam: Camera, world: &World) -> Color {

    let mut rng = rand::thread_rng();
    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

    for _s in 0..SAMPLES_PER_PIXEL {
        let u = (x as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH-1) as f64;
        let v = (y as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT-1) as f64;
        let r: Ray = cam.get_ray(u, v);
        let color = ray_color(r, world, MAX_DEPTH);
        pixel_color = pixel_color + color;
    }

    pixel_color

}

fn main() {

    let mut buffer: Vec<Color> = vec![Color{r: 0.0, g:0.0, b:0.0}; (IMAGE_WIDTH*IMAGE_HEIGHT) as usize];

    let lookfrom: Vec3 = Vec3::new(13.0,2.0,3.0);
    let lookat: Vec3 = Vec3::new(0.0,0.0,0.0);
    let vup: Vec3 = Vec3::new(0.0,1.0,0.0);


    let mut cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO,
                              0.1, // Aperture
                              10.0); // Dist to focus


    println!("Image {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);


    let term_w: usize;
    if let Some((w, _h)) = term_size::dimensions() {
        term_w = w;
    } else {
        term_w = 10;
    }


    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = mpsc::channel();


    let seed: u64 = 7;

    let mut angle: f64 = 0.0;
    let sx: f64 = 15.0;
    let sz: f64 = 3.0;
    for i in 0..1 {

        let start_time = Instant::now();

        angle+=3.6;
        let cx = sx * f64::cos(angle.to_radians()) - sz*f64::sin(angle.to_radians());
        let cz = sx * f64::sin(angle.to_radians()) - sz*f64::cos(angle.to_radians());
        cam.set_position(Vec3::new(cx, 2.0, cz));


        for y in (0..IMAGE_HEIGHT).rev() {
            let world = create_world(seed);

            let tx2 = tx.clone();
            pool.execute(move|| {
                for x in 0..IMAGE_WIDTH {
                    let pixel_color = compute_pixel(x, y, cam, &world);
                    tx2.send((x,y, pixel_color)).unwrap();
                }
            });
        }


        let mut pixel_count: u32 = 0;
        for received in &rx {
            let (tx, ty, tc) = received;
            pixel_count+=1;
            if pixel_count == IMAGE_WIDTH*IMAGE_HEIGHT {
                break;
            }
            put_pixel(&mut buffer, tx, ty, tc);
            if (pixel_count%IMAGE_HEIGHT)==0 {
                print_progress(term_w, (pixel_count+1) as f64 / (IMAGE_WIDTH*IMAGE_HEIGHT) as f64);
            }
        }

        println!("");

        let elapsed_time = start_time.elapsed();
        println!("{}s", ((elapsed_time.as_secs()*1000)+elapsed_time.subsec_millis() as u64) as f64 / 1000.0);
        write_image(&format!("test_{:04}.png", i).to_string(), IMAGE_WIDTH, IMAGE_HEIGHT, &mut buffer);

    }


}
