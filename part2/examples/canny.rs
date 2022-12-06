use image::{ImageError};
use rust_for_multimedia_canny::{drog::perform_drog_convolution};

fn main() -> Result<(), ImageError> {
    // Grayscale conversion
    let image_reader = image::io::Reader::open("assets/myownlena.jpg")?;
    let image = image_reader.decode()?;
    let image_luma = image.into_luma8();
    image_luma.save("outputs/myownlena_luma8.jpg").unwrap();

    // DroG convolution
    let sigma = 2.0;
    let kernel_size = 10;
    perform_drog_convolution(&image_luma, kernel_size, sigma);

    Ok(())
}