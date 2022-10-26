use test::Bencher;

use crate::convert;

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