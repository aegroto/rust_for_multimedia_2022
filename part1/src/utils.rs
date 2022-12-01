pub fn allocate_yuv_buffers(width: u32, height: u32) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let pixels_count = (width * height) as usize;
    let y_pixels = vec![0u8; pixels_count];
    let u_pixels = vec![0u8; pixels_count];
    let v_pixels = vec![0u8; pixels_count];

    (y_pixels, u_pixels, v_pixels)
}

