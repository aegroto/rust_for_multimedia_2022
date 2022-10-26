#![feature(test)]
extern crate test;

use image::GrayImage;
use itertools::{izip, Itertools};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use utils::allocate_yuv_buffers;

#[cfg(test)]
mod benches;

#[cfg(test)]
mod tests;

mod utils;

fn main() {
    let image = image::io::Reader::open("assets/myownlena.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let rgb_pixels = image.as_bytes();
    let (mut y_pixels, mut u_pixels, mut v_pixels) =
        allocate_yuv_buffers(image.width(), image.height());

    convert_parallel(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels, 16);

    save_grayscale("planes/y.png", y_pixels, image.width(), image.height());
    save_grayscale("planes/u.png", u_pixels, image.width(), image.height());
    save_grayscale("planes/v.png", v_pixels, image.width(), image.height());
}

fn save_grayscale(path: &str, pixels: Vec<u8>, width: u32, height: u32) {
    let plane = GrayImage::from_raw(width, height, pixels).unwrap();
    plane.save(path).unwrap();
}

pub fn convert(rgb_pixels: &[u8], y_pixels: &mut [u8], u_pixels: &mut [u8], v_pixels: &mut [u8]) {
    for i in 0..y_pixels.len() {
        let (r, g, b) = (
            rgb_pixels[i * 3],
            rgb_pixels[i * 3 + 1],
            rgb_pixels[i * 3 + 2],
        );
        let (y, u, v) = rgb_to_yuv(r, g, b);

        y_pixels[i] = y;
        u_pixels[i] = u;
        v_pixels[i] = v;
    }
}

pub fn convert_tuples(
    rgb_data: &[u8],
    y_pixels: &mut [u8],
    u_pixels: &mut [u8],
    v_pixels: &mut [u8],
) {
    let rgb_pixels = rgb_data.iter().tuples();

    for (i, (r, g, b)) in rgb_pixels.enumerate() {
        let (y, u, v) = rgb_to_yuv(*r, *g, *b);

        y_pixels[i] = y;
        u_pixels[i] = u;
        v_pixels[i] = v;
    }
}

pub fn convert_tuples_indexless(
    rgb_data: &[u8],
    y_pixels: &mut [u8],
    u_pixels: &mut [u8],
    v_pixels: &mut [u8],
) {
    let rgb_pixels = rgb_data.iter().tuples();

    let iter = izip!(rgb_pixels, y_pixels, u_pixels, v_pixels);

    for ((r_ref, g_ref, b_ref), y_ref, u_ref, v_ref) in iter {
        let (y, u, v) = rgb_to_yuv(*r_ref, *g_ref, *b_ref);
        *y_ref = y;
        *u_ref = u;
        *v_ref = v;
    }
}

pub fn convert_iter(
    rgb_pixels: &[u8],
    y_pixels: &mut [u8],
    u_pixels: &mut [u8],
    v_pixels: &mut [u8],
) {
    (0..y_pixels.len()).into_iter().for_each(|i| {
        let (r, g, b) = (
            rgb_pixels[i * 3],
            rgb_pixels[i * 3 + 1],
            rgb_pixels[i * 3 + 2],
        );
        let (y, u, v) = rgb_to_yuv(r, g, b);

        y_pixels[i] = y;
        u_pixels[i] = u;
        v_pixels[i] = v;
    });
}

pub fn convert_parallel(
    rgb_pixels: &[u8],
    y_pixels: &mut [u8],
    u_pixels: &mut [u8],
    v_pixels: &mut [u8],
    chunks_count: usize,
) {
    let rgb_chunks = rgb_pixels.chunks(rgb_pixels.len() / chunks_count);
    let y_chunks = y_pixels.chunks_mut(y_pixels.len() / chunks_count);
    let u_chunks = u_pixels.chunks_mut(u_pixels.len() / chunks_count);
    let v_chunks = v_pixels.chunks_mut(v_pixels.len() / chunks_count);

    let iter = izip!(rgb_chunks, y_chunks, u_chunks, v_chunks);

    iter.par_bridge()
        .for_each(|(rgb_chunk, y_chunk, u_chunk, v_chunk)| {
            convert(rgb_chunk, y_chunk, u_chunk, v_chunk);
        });
}

pub fn convert_tuples_parallel(
    rgb_pixels: &[u8],
    y_pixels: &mut [u8],
    u_pixels: &mut [u8],
    v_pixels: &mut [u8],
    chunks_count: usize,
) {
    let rgb_chunks = rgb_pixels.chunks(rgb_pixels.len() / chunks_count);
    let y_chunks = y_pixels.chunks_mut(y_pixels.len() / chunks_count);
    let u_chunks = u_pixels.chunks_mut(u_pixels.len() / chunks_count);
    let v_chunks = v_pixels.chunks_mut(v_pixels.len() / chunks_count);

    let iter = izip!(rgb_chunks, y_chunks, u_chunks, v_chunks);

    iter.par_bridge()
        .for_each(|(rgb_chunk, y_chunk, u_chunk, v_chunk)| {
            convert_tuples(rgb_chunk, y_chunk, u_chunk, v_chunk);
        });
}

fn rgb_to_yuv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let y = r * 0.29900 + g * 0.58700 + b * 0.11400;
    let u = (r * -0.16874 + g * -0.33126 + b * 0.50000) + 128.0;
    let v = (r * 0.50000 + g * -0.41869 + b * -0.08131) + 128.0;

    (y as u8, u as u8, v as u8)
}
