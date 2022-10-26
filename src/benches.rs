use image::DynamicImage;
use test::Bencher;

use crate::{convert, convert_iter, convert_parallel, convert_tuples, convert_tuples_indexless, utils::allocate_yuv_buffers};

fn load_image() -> DynamicImage {
    image::io::Reader::open("assets/myownlena.jpg")
        .unwrap()
        .decode()
        .unwrap()
}

#[bench]
fn bench_convert(b: &mut Bencher) {
    let image = load_image();
    let rgb_pixels = image.as_bytes();
    let (mut y_pixels, mut u_pixels, mut v_pixels) =
        allocate_yuv_buffers(image.width(), image.height());

    b.iter(|| convert(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels));
}

#[bench]
fn bench_convert_tuples(b: &mut Bencher) {
    let image = load_image();
    let rgb_pixels = image.as_bytes();
    let (mut y_pixels, mut u_pixels, mut v_pixels) =
        allocate_yuv_buffers(image.width(), image.height());

    b.iter(|| convert_tuples(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels));
}

#[bench]
fn bench_convert_tuples_indexless(b: &mut Bencher) {
    let image = load_image();
    let rgb_pixels = image.as_bytes();
    let (mut y_pixels, mut u_pixels, mut v_pixels) =
        allocate_yuv_buffers(image.width(), image.height());

    b.iter(|| convert_tuples_indexless(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels));
}

#[bench]
fn bench_convert_iter(b: &mut Bencher) {
    let image = load_image();
    let rgb_pixels = image.as_bytes();
    let (mut y_pixels, mut u_pixels, mut v_pixels) =
        allocate_yuv_buffers(image.width(), image.height());

    b.iter(|| convert_iter(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels));
}

#[bench]
fn bench_convert_parallel_16_chunks(b: &mut Bencher) {
    let image = load_image();
    let rgb_pixels = image.as_bytes();
    let (mut y_pixels, mut u_pixels, mut v_pixels) =
        allocate_yuv_buffers(image.width(), image.height());

    b.iter(|| convert_parallel(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels, 16));
}
