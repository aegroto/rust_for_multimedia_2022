use std::io::Cursor;

use image::{DynamicImage, GrayImage, ImageFormat};

fn main() {
    let image = image::io::Reader::open("assets/myownlena.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let rgb_pixels = image.as_bytes();
    let pixels_count = (image.width() * image.height()) as usize;
    let mut y_pixels = vec![0u8; pixels_count];
    let mut u_pixels = vec![0u8; pixels_count];
    let mut v_pixels = vec![0u8; pixels_count];

    convert(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels);

    save_grayscale("planes/y.png", y_pixels, image.width(), image.height());
    save_grayscale("planes/u.png", u_pixels, image.width(), image.height());
    save_grayscale("planes/v.png", v_pixels, image.width(), image.height());
}

fn save_grayscale(path: &str, pixels: Vec<u8>, width: u32, height: u32) {
    let plane = GrayImage::from_raw(width, height, pixels).unwrap();
    plane.save(path).unwrap();
}

fn convert(rgb_pixels: &[u8], y_pixels: &mut [u8], u_pixels: &mut [u8], v_pixels: &mut [u8]) {
    for i in 0..y_pixels.len() {
        let (b, g, r) = (
            rgb_pixels[i * 3],
            rgb_pixels[i * 3 + 1],
            rgb_pixels[i * 3 + 2],
        );
        let (y, u, v) = bgr_to_yuv_f32(b, g, r);

        y_pixels[i] = y as u8;
        u_pixels[i] = u as u8;
        v_pixels[i] = v as u8;
    }
}

fn bgr_to_yuv_f32(b: u8, g: u8, r: u8) -> (f32, f32, f32) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let y = r * 0.29900 + g * 0.58700 + b * 0.11400;
    let u = (r * -0.16874 + g * -0.33126 + b * 0.50000) + 128.0;
    let v = (r * 0.50000 + g * -0.41869 + b * -0.08131) + 128.0;

    (y, u, v)
}
