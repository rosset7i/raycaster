use glium::implement_vertex;

use crate::consts::{MAP, TILE_SIZE};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub fn draw_player(x: f32, y: f32) -> Vec<Vertex> {
    let color = [0.0, 1.0, 0.0];

    let center = Vertex {
        position: [x, y],
        color,
    };
    let top = Vertex {
        position: [x, y + 15.0],
        color,
    };
    let right = Vertex {
        position: [x + 15.0, y],
        color,
    };
    let left = Vertex {
        position: [x - 15.0, y],
        color,
    };
    let bottom = Vertex {
        position: [x, y - 15.0],
        color,
    };

    vec![
        center, top, left, center, bottom, left, center, bottom, right, center, top, right, top,
    ]
}

fn squarify(x: f32, y: f32) -> Vec<Vertex> {
    let color = [1.0, 0.0, 0.0];

    let x = x * TILE_SIZE;
    let y = y * TILE_SIZE;

    let bottom_left = Vertex {
        position: [x + 1.0, y + 1.0],
        color,
    };
    let top_left = Vertex {
        position: [x + 1.0, y + TILE_SIZE - 1.0],
        color,
    };
    let top_right = Vertex {
        position: [x + TILE_SIZE - 1.0, y + TILE_SIZE - 1.0],
        color,
    };
    let bottom_right = Vertex {
        position: [x + TILE_SIZE - 1.0, y + 1.0],
        color,
    };

    vec![
        bottom_left,
        top_left,
        top_right,
        top_right,
        bottom_right,
        bottom_left,
    ]
}

pub fn draw_map() -> Vec<Vertex> {
    MAP.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, value)| {
                if *value != 0 {
                    Some(squarify(x as f32, y as f32))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}
