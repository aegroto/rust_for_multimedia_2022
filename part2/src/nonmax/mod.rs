use crate::{edge::Edge, Matrix};

pub fn perform_nonmax_suppression(
    edges: &Matrix<Edge>,
    distance_range: usize
) -> Matrix<Edge> {
    let mut filtered_edges = edges.clone();
    for x in 0..edges.width() {
        for y in 0..edges.height() {
            if !is_max(edges, x, y, distance_range) {
                filtered_edges.set(x, y, Edge::zero());
            }
        }
    }

    filtered_edges
}

fn is_max(
    edges: &Matrix<Edge>,
    x: usize,
    y: usize,
    distance_range: usize,
) -> bool {
    let edge = edges.get(x, y);
    let distance_range = distance_range as i32;

    for distance in -distance_range..distance_range {
        let (dir_x, dir_y) = edge.dir_norm();

        let (near_x_offset, near_y_offset): (i32, i32) = (
            (dir_x.signum() as i32) * (if dir_x.abs() > 0.25 { distance } else { 0 }),
            (dir_y.signum() as i32) * (if dir_y.abs() > 0.25 { distance } else { 0 }),
        );

        let near_x: i32 = x as i32 + near_x_offset;
        let near_y: i32 = y as i32 + near_y_offset;

        if near_x < 0 || near_y < 0 { 
            continue;
        }

        let near_x = near_x as usize;
        let near_y = near_y as usize;

        if near_x >= edges.width() || near_y >= edges.height() {
            continue;
        }

        let near_edge = edges.get(near_x, near_y);

        if edge.get_magnitude() < near_edge.get_magnitude() {
            return false;
        }
    }

    true
}
