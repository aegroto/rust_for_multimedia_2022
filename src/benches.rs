use test::Bencher;

use crate::{convert, convert_iter, convert_parallel};

#[bench]
fn bench_convert(b: &mut Bencher) {
    let image = image::io::Reader::open("assets/myownlena.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let rgb_pixels = image.as_bytes();
    let pixels_count = (image.width() * image.height()) as usize;
    let mut y_pixels = vec![0u8; pixels_count];
    let mut u_pixels = vec![0u8; pixels_count];
    let mut v_pixels = vec![0u8; pixels_count];

    b.iter(|| convert(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels));
}

#[bench]
fn bench_convert_iter(b: &mut Bencher) {
    let image = image::io::Reader::open("assets/myownlena.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let rgb_pixels = image.as_bytes();
    let pixels_count = (image.width() * image.height()) as usize;
    let mut y_pixels = vec![0u8; pixels_count];
    let mut u_pixels = vec![0u8; pixels_count];
    let mut v_pixels = vec![0u8; pixels_count];

    b.iter(|| convert_iter(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels));
}

#[bench]
fn bench_convert_parallel_16_chunks(b: &mut Bencher) {
    let image = image::io::Reader::open("assets/myownlena.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let rgb_pixels = image.as_bytes();
    let pixels_count = (image.width() * image.height()) as usize;
    let mut y_pixels = vec![0u8; pixels_count];
    let mut u_pixels = vec![0u8; pixels_count];
    let mut v_pixels = vec![0u8; pixels_count];

    b.iter(|| convert_parallel(rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels, 16));
}


