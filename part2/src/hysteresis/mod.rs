use itertools::Itertools;

use crate::{
    edge::{Edge, ThresholdedEdge},
    Matrix,
};

pub fn perform_hysteresis_thresholding(
    edges: &Matrix<Edge>,
    weak_edge_threshold: f64,
    strong_edge_threshold: f64,
    neighbourhood_range: usize,
) -> Matrix<ThresholdedEdge> {
    let mut thresholded_edges = Matrix::new(
        vec![ThresholdedEdge::NULL; edges.values().len()],
        edges.width,
        edges.height,
    );

    for x in 0..edges.width() {
        for y in 0..edges.height() {
            let magnitude = edges.get(x, y).get_magnitude();

            let thresholded_value = if magnitude < strong_edge_threshold {
                ThresholdedEdge::NULL
            } else if magnitude < weak_edge_threshold {
                ThresholdedEdge::WEAK
            } else {
                ThresholdedEdge::STRONG
            };

            thresholded_edges.set(x, y, thresholded_value);
        }
    }

    let thresholded_edges = thresholded_edges;
    let mut hysteresis_thresholded_edges = thresholded_edges.clone();

    for x in 0..thresholded_edges.width() {
        for y in 0..thresholded_edges.height() {
            let edge = hysteresis_thresholded_edges.get(x, y);

            if !matches!(edge, ThresholdedEdge::WEAK) {
                continue;
            }

            if has_strong_neighbour(&thresholded_edges, x, y, neighbourhood_range) {
                hysteresis_thresholded_edges.set(x, y, ThresholdedEdge::STRONG);
            }
        }
    }

    hysteresis_thresholded_edges
}

fn has_strong_neighbour(
    edges: &Matrix<ThresholdedEdge>,
    x: usize,
    y: usize,
    range: usize,
) -> bool {
    let x = x as i32;
    let y = y as i32;
    let range = range as i32;

    let neighbourhood_indices = (x - range..x + range)
        .cartesian_product(y - range..y + range);

    for (nx, ny) in neighbourhood_indices {
        if nx < 0 || ny < 0 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if nx >= edges.width() || ny >= edges.height() {
            continue;
        }

        let neighbour_edge = edges.get(nx, ny);
        if matches!(neighbour_edge, ThresholdedEdge::STRONG) {
            return true;
        }
    }

    return false;
}
