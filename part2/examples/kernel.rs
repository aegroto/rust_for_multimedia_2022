use rust_for_multimedia_canny::drog::kernel;

fn main() {
    let x_kernel = kernel::drog_x(3, 0.5);
    println!("X Kernel: ");
    println!("{:?}", x_kernel);

    // println!("Y Kernel: ");
    // println!("{:?}", y_kernel);
}
