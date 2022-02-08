#![allow(non_snake_case)]
use std::fs::File;
use std::io::prelude::*;
extern crate image;

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


fn write_image_ppm(filename: &str, w: u32, h: u32, buffer: &[Color]) -> std::io::Result<()> {
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

fn write_image(filename: &str, w: u32, h: u32, buffer: &[Color])  {
    let mut buf = vec![0; buffer.len()*3];
    for i in 0..buffer.len()-3 {
        buf[i*3] = (buffer[i].r*255.0) as u8;
        buf[(i*3)+1] = (buffer[i].g*255.0) as u8;
        buf[(i*3)+2] = (buffer[i].b*255.0) as u8;
    }
    image::save_buffer(filename, &buf, w, h, image::ColorType::Rgb8).unwrap()
}

fn main() {

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:  u32 = 400;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as u32;

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

    write_image_ppm("test.ppm", IMAGE_WIDTH, IMAGE_HEIGHT, &buffer).ok();
    write_image("test.png", IMAGE_WIDTH, IMAGE_HEIGHT, &buffer);
}
