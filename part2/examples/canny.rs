use image::{ImageError, GrayImage};
use rust_for_multimedia_canny::{gradient::{perform_gaussian_plus_sobel_convolution}, edge::Edge, denormalize, nonmax::perform_nonmax_suppression, hysteresis::perform_hysteresis_thresholding, thresholded_edge_to_pixel};

fn main() -> Result<(), ImageError> {
    // Grayscale conversion
    println!("Converting to grayscale...");
    let image_reader = image::io::Reader::open("assets/lena.png")?;
    let image = image_reader.decode()?;
    let image_luma = image.into_luma8();
    image_luma.save("outputs/luma8.png").unwrap();

    // Gradients calculation
    println!("Calculating the gradients...");
    let sigma = 2.0;
    let kernel_size = 3;
    let gradient_edges = perform_gaussian_plus_sobel_convolution(&image_luma, kernel_size, sigma);
    GrayImage::from_vec(
        image_luma.width(),
        image_luma.height(),
        gradient_edges.values().iter()
            .map(Edge::get_magnitude)
            .map(denormalize)
            .collect()
    )    
    .unwrap()
    .save("outputs/gradient_magnitude.png")
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
    .save("outputs/max_edges.png")
    .unwrap();

    println!("Applying hysteresis thresholding...");
    let thresholded_edges = perform_hysteresis_thresholding(&max_edges, 0.05, 0.1, 1);
    GrayImage::from_vec(
        thresholded_edges.width() as u32,
        thresholded_edges.height() as u32,
        thresholded_edges.values().iter()
            .map(|value| thresholded_edge_to_pixel(*value))
            .collect()
    )    
    .unwrap()
    .save("outputs/thresholded_edges.png")
    .unwrap();

    Ok(())
}
