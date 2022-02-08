#![allow(non_snake_case)]
use std::fs::File;
use std::io::prelude::*;

pub mod vec3 {
    include!("vec3.rs");
}
use vec3::Vec3;
#[derive(Clone)]
#[derive(Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}


fn write_image(filename: &str, w: u32, h: u32, buffer: &[Color]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write(b"P3\n")?;
    file.write(format!("{} {}\n", w, h).as_bytes())?;
    file.write(b"255\n")?;

    for y in (0..h).rev() {
        for x in 0..w {
            file.write(format!("{} {} {}\n",
                               (buffer[(x+y*w) as usize].r*255.999) as u32,
                               (buffer[(x+y*w) as usize].g*255.999) as u32,
                               (buffer[(x+y*w) as usize].b*255.999) as u32).as_bytes())?;
        }
    }
    Ok(())
}

fn main() {

    const IMAGE_WIDTH:  u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut buffer = [Color{r: 0.0, g:0.0, b:0.0}; (IMAGE_WIDTH*IMAGE_HEIGHT) as usize];

    println!("Image {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);


    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let r = (x as f64) / (IMAGE_WIDTH-1) as f64;
            let g = (y as f64) / (IMAGE_HEIGHT-1) as f64;
            let b = 0.25;

            buffer[(x+y*IMAGE_WIDTH) as usize].r = r;
            buffer[(x+y*IMAGE_WIDTH) as usize].g = g;
            buffer[(x+y*IMAGE_WIDTH) as usize].b = b;
        }
    }

    let v1: Vec3 = Vec3::new(2.0, 3.0, 4.0);
    let v2: Vec3 = Vec3::new(5.0, 6.0, 7.0);
    let v3: Vec3 = v1*v2;

    println!("Vec: {}", v1.cross(v2));
    write_image("test.ppm", IMAGE_WIDTH, IMAGE_HEIGHT, &buffer).ok();
}
