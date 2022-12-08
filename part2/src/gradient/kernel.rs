use crate::Matrix;

pub fn gaussian(size: usize, sigma: f64) -> Matrix<f64> {
    let mut kernel = Matrix::new(
        vec![0.0; size * size], size, size
    );

    let dev = sigma * sigma;

    let mut sum = 0.0;

    for x in 0..size {
        for y in 0..size {
            let xf = (x as i32 - (size/2) as i32) as f64;
            let yf = (y as i32 - (size/2) as i32) as f64;
            let value = f64::exp(-(xf*xf + yf*yf) / (2.0 * dev));
            kernel.set(y, x, value);

            sum += value;
        }
    }

    for x in 0..size {
        for y in 0..size {
            let normalized_value = kernel.get(x, y) / sum;
            kernel.set(x, y, normalized_value);
        }
    }

    kernel
}