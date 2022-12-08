use rust_for_multimedia_canny::gradient::kernel;

fn main() {
    let gaussian_kernel = kernel::gaussian(3, 0.5);
    println!("Gaussian kernel: ");
    println!("{:?}", gaussian_kernel);
}
