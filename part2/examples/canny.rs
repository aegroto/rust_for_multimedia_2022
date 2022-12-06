use image::{ImageError, GrayImage};
use rust_for_multimedia_canny::{drog::perform_drog_convolution, edge::Edge, denormalize};

fn main() -> Result<(), ImageError> {
    // Grayscale conversion
    println!("Converting to grayscale...");
    let image_reader = image::io::Reader::open("assets/myownlena.jpg")?;
    let image = image_reader.decode()?;
    let image_luma = image.into_luma8();
    image_luma.save("outputs/myownlena_luma8.png").unwrap();

    // DroG convolution
    println!("Applying DroG convolution...");
    let sigma = 2.0;
    let kernel_size = 10;
    let gradient_edges = perform_drog_convolution(&image_luma, kernel_size, sigma);
    GrayImage::from_vec(
        image_luma.width(),
        image_luma.height(),
        gradient_edges.values().iter()
            .map(Edge::get_magnitude)
            .map(denormalize)
            .collect()
    )    
    .unwrap()
    .save("outputs/myownlena_drog_magnitude.png")
    .unwrap();

    Ok(())
}
