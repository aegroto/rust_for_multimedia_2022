use image::{ImageError, GrayImage};
use rust_for_multimedia_canny::{drog::{perform_drog_convolution}, edge::Edge, denormalize, nonmax::perform_nonmax_suppression, hysteresis::perform_hysteresis_thresholding, thresholded_edge_to_pixel};

fn main() -> Result<(), ImageError> {
    // Grayscale conversion
    println!("Converting to grayscale...");
    let image_reader = image::io::Reader::open("assets/myownlena.jpg")?;
    let image = image_reader.decode()?;
    let image_luma = image.into_luma8();
    image_luma.save("outputs/myownlena_luma8.png").unwrap();

    // DroG convolution
    println!("Applying DroG convolution...");
    let sigma = 0.5;
    let kernel_size = 3;
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
    let max_edges = perform_nonmax_suppression(&gradient_edges, 2);
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

    println!("Applying hysteresis thresholding...");
    let thresholded_edges = perform_hysteresis_thresholding(&max_edges, 0.02, 0.05, 1);
    GrayImage::from_vec(
        thresholded_edges.width() as u32,
        thresholded_edges.height() as u32,
        thresholded_edges.values().iter()
            .map(|value| thresholded_edge_to_pixel(*value))
            .collect()
    )    
    .unwrap()
    .save("outputs/myownlena_thresholded_edges.png")
    .unwrap();

    Ok(())
}
