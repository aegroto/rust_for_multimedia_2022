use crate::*;

fn load_test_data() -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let rgb_pixels = [100, 200, 50, 0, 20, 250, 120, 50, 20, 70, 54, 65];

    let expected_y_pixels = [153, 40, 67, 60];
    let expected_u_pixels = [69, 246, 101, 130];
    let expected_v_pixels = [90, 99, 165, 135];

    (
        rgb_pixels.to_vec(),
        expected_y_pixels.to_vec(),
        expected_u_pixels.to_vec(),
        expected_v_pixels.to_vec(),
    )
}

#[test]
fn test_convert_4_pixels() {
    let (rgb_pixels, expected_y_pixels, expected_u_pixels, expected_v_pixels) = load_test_data();
    let (mut y_pixels, mut u_pixels, mut v_pixels) = allocate_yuv_buffers(2, 2);

    convert(&rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels);

    assert_eq!(y_pixels, expected_y_pixels);
    assert_eq!(u_pixels, expected_u_pixels);
    assert_eq!(v_pixels, expected_v_pixels);
}

#[test]
fn test_convert_tuples_4_pixels() {
    let (rgb_pixels, expected_y_pixels, expected_u_pixels, expected_v_pixels) = load_test_data();

    let (mut y_pixels, mut u_pixels, mut v_pixels) = allocate_yuv_buffers(2, 2);

    convert_tuples(&rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels);

    assert_eq!(y_pixels, expected_y_pixels);
    assert_eq!(u_pixels, expected_u_pixels);
    assert_eq!(v_pixels, expected_v_pixels);
}

#[test]
fn test_convert_tuples_indexless_4_pixels() {
    let (rgb_pixels, expected_y_pixels, expected_u_pixels, expected_v_pixels) = load_test_data();

    let (mut y_pixels, mut u_pixels, mut v_pixels) = allocate_yuv_buffers(2, 2);

    convert_tuples_indexless(&rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels);

    assert_eq!(y_pixels, expected_y_pixels);
    assert_eq!(u_pixels, expected_u_pixels);
    assert_eq!(v_pixels, expected_v_pixels);
}

#[test]
fn test_convert_iter_4_pixels() {
    let (rgb_pixels, expected_y_pixels, expected_u_pixels, expected_v_pixels) = load_test_data();

    let (mut y_pixels, mut u_pixels, mut v_pixels) = allocate_yuv_buffers(2, 2);

    convert_iter(&rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels);

    assert_eq!(y_pixels, expected_y_pixels);
    assert_eq!(u_pixels, expected_u_pixels);
    assert_eq!(v_pixels, expected_v_pixels);
}

#[test]
fn test_convert_parallel_4_pixels() {
    let (rgb_pixels, expected_y_pixels, expected_u_pixels, expected_v_pixels) = load_test_data();

    let (mut y_pixels, mut u_pixels, mut v_pixels) = allocate_yuv_buffers(2, 2);

    convert_parallel(&rgb_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels, 2);

    assert_eq!(y_pixels, expected_y_pixels);
    assert_eq!(u_pixels, expected_u_pixels);
    assert_eq!(v_pixels, expected_v_pixels);
}
