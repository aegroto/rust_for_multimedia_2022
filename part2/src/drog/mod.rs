use image::GrayImage;

use crate::{denormalize, edge::Edge, normalize, Matrix};

mod kernel;

pub fn perform_drog_convolution(image: &GrayImage, kernel_size: usize, sigma: f64) -> Matrix<Edge> {
    let pixels = Matrix::new(
        image
            .as_raw()
            .iter()
            .map(|p| normalize(*p))
            .collect::<Vec<f64>>(),
        image.width() as usize,
        image.height() as usize,
    );
    let (drog_x_kernel, drog_y_kernel) = kernel::drog(kernel_size, sigma);

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

    let drog_y_pixels = perform_convolution(&pixels, &drog_y_kernel);
    GrayImage::from_vec(
        drog_y_pixels.width() as u32,
        drog_y_pixels.height() as u32,
        drog_y_pixels
            .values()
            .iter()
            .map(|p| denormalize(*p))
            .collect(),
    )
    .unwrap()
    .save("outputs/myownlena_drog_y.png")
    .unwrap();

    let edge_values = drog_x_pixels
        .values()
        .iter()
        .zip(drog_y_pixels.values().iter())
        .map(|(x, y)| Edge::new(*x, *y))
        .collect::<Vec<Edge>>();

    Matrix::new(edge_values, image.width() as usize, image.height() as usize)
}

pub fn perform_convolution(pixels: &Matrix<f64>, kernel: &Matrix<f64>) -> Matrix<f64> {
    let mut convoluted_pixels = pixels.clone();

    let flipped_kernel = kernel.flipped();

    for pixel_x in 0..pixels.width() as i32 {
        for pixel_y in 0..pixels.height() as i32 {
            let mut convolution_sum = 0.0;

            for kernel_x in 0..flipped_kernel.width() as i32 {
                for kernel_y in 0..flipped_kernel.height() as i32 {
                    let x_offset = kernel_x - flipped_kernel.width() as i32 / 2;
                    let y_offset = kernel_y - flipped_kernel.height() as i32 / 2;

                    if pixel_x + x_offset < 0
                        || pixel_y + y_offset < 0
                        || pixel_x + x_offset >= pixels.width() as i32
                        || pixel_y + y_offset >= pixels.height() as i32
                    {
                        continue;
                    }

                    let kernel_value = flipped_kernel.get(kernel_x as usize, kernel_y as usize);

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
        let kernel = Matrix::new(vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0], 3, 3);

        let mut image = Matrix::new(
            vec![
                0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
                0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
            ],
            5,
            5,
        );

        perform_convolution(&mut image, &kernel);

        println!("{:?}", image);
    }
}
