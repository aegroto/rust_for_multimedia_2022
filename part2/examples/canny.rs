use image::{ImageError, GrayImage};
use rust_for_multimedia_canny::{drog::perform_drog_convolution, edge::Edge, denormalize, nonmax::perform_nonmax_suppression};

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

    println!("Applying non-maximum suppression...");
    let max_edges = perform_nonmax_suppression(&gradient_edges, 3);
    println!(
        "Non-zero magnitudes: {}",
        max_edges
            .values()
            .iter()
            .filter(|edge| edge.get_magnitude() > 0.0)
            .count()
    );
    GrayImage::from_vec(
        image_luma.width(),
        image_luma.height(),
        max_edges.values().iter()
            .map(Edge::get_magnitude)
            .map(denormalize)
            .collect()
    )    
    .unwrap()
    .save("outputs/myownlena_max_edges.png")
    .unwrap();

    Ok(())
}
