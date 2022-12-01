use image::GrayImage;

use crate::{denormalize, normalize, Matrix};

mod kernel;

pub fn perform_drog_convolution(image: &GrayImage, kernel_size: usize, sigma: f64) {
    let pixels = Matrix::new(
        image
            .as_raw()
            .iter()
            .map(|p| normalize(*p))
            .collect::<Vec<f64>>(),
        image.width() as usize,
        image.height() as usize,
    );
    let (drog_x_kernel, _drog_y_kernel) = kernel::drog(kernel_size, sigma);
    // println!("Drog kernel x: {:#?}", drog_x_kernel.values());

    /*let drog_x_kernel = Matrix::new(vec![
        -1.0, 0.0, 1.0,
        -2.0, 0.0, 2.0,
        -1.0, 0.0, 1.0,
    ], 3, 3);*/

    let drog_x_pixels = perform_convolution(&pixels, &drog_x_kernel);
    GrayImage::from_vec(
        drog_x_pixels.width() as u32,
        drog_x_pixels.height() as u32,
        drog_x_pixels
            .values()
            .iter()
            .map(|p| denormalize(*p))
            .collect(),
    )
    .unwrap()
    .save("outputs/myownlena_drog_x.png")
    .unwrap();
}

pub fn perform_convolution(pixels: &Matrix<f64>, kernel: &Matrix<f64>) -> Matrix<f64> {
    let mut convoluted_pixels = pixels.clone();

    for pixel_x in 0..pixels.width() as i32 {
        for pixel_y in 0..pixels.height() as i32 {
            let mut convolution_sum = 0.0;

            for kernel_x in 0..kernel.width() as i32 {
                for kernel_y in 0..kernel.height() as i32 {
                    let x_offset = kernel_x - kernel.width() as i32 / 2;
                    let y_offset = kernel_y - kernel.height() as i32 / 2;

                    if pixel_x + x_offset < 0
                        || pixel_y + y_offset < 0
                        || pixel_x + x_offset >= pixels.width() as i32
                        || pixel_y + y_offset >= pixels.height() as i32
                    {
                        continue;
                    }

                    let kernel_value = kernel.get(kernel_x as usize, kernel_y as usize);

                    let pixel_value =
                        pixels.get((pixel_x + x_offset) as usize, (pixel_y + y_offset) as usize);

                    convolution_sum += pixel_value * kernel_value;
                }
            }

            // println!("{}", pixels.get(pixel_x as usize, pixel_y as usize));
            // println!("{}", convolution_sum);
            convoluted_pixels.set(pixel_x as usize, pixel_y as usize, convolution_sum);
            // println!("Pixels: ");
            // println!("{:?}", convoluted_pixels);
        }
    }

    convoluted_pixels
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    use super::perform_convolution;

    #[test]
    fn test_conv() {
        let kernel = Matrix::new(
            vec![
                -1.0, 0.0, 1.0,
                -2.0, 0.0, 2.0,
                -1.0, 0.0, 1.0,
            ], 3, 3
        );

        let mut image = Matrix::new(
            vec![
                0.5, 0.5, 0.5, 0.5, 0.5,
                0.5, 0.5, 0.5, 0.5, 0.5,
                0.5, 0.5, 0.5, 0.5, 0.5,
                0.5, 0.5, 0.5, 0.5, 0.5,
                0.5, 0.5, 0.5, 0.5, 0.5,
            ], 5, 5
        );

        perform_convolution(&mut image, &kernel);

        println!("{:?}", image);
    }
}
