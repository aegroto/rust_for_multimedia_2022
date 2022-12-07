use std::f64::consts::PI;

use crate::Matrix;

pub fn drog_x(size: usize, sigma: f64) -> Matrix<f64> {
    let mut kernel = Matrix::new(
        vec![0.0; size * size], size, size
    );

    let dev = sigma * sigma;

    let mut sum = 0.0;

    for x in 0..size {
        for y in 0..size {
            let xf = (x as i32 - (size/2) as i32) as f64;
            let yf = (y as i32 - (size/2) as i32) as f64;

            // let gaussian_sample = (1.0 / ((2.0 * PI).sqrt() * dev)) * f64::exp(-(xf*xf + yf*yf) / (2.0 * dev));
            let gaussian_sample = f64::exp(-(xf*xf + yf*yf) / (2.0 * dev));
            // let value = -(xf/dev) * gaussian_sample;
            let value = -(xf/dev) * gaussian_sample;
            kernel.set(y, x, value);

            sum += value.abs();
        }
    }

    println!("{}", sum);

    let normalization_factor = 1.0; 
    println!("{}", normalization_factor);

    for x in 0..size {
        for y in 0..size {
            let normalized_value = kernel.get(x, y) * normalization_factor;

            // let xf = (x as i32 - (size/2) as i32) as f64;
            // let value = -(xf/(dev)) * normalized_value;
            let value = normalized_value;

            kernel.set(x, y, value);
        }
    }

    kernel
}

pub fn drog_y(size: usize, sigma: f64) -> Matrix<f64> {
    drog_x(size, sigma).transposed()
}