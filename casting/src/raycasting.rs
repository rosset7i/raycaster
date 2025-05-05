use crate::{
    map::Map,
    math::{Angle, HALF_CIRCUNFERENCE, ONE_FORTH_CIRCUNFERENCE, THREE_FORTH_CIRCUNFERENCE},
    player::PlayerPosition,
};

pub const OFFSET: f32 = 0.0001;
pub const DEPTH_OF_FIELD: u32 = 10;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

pub fn get_player_vertices(player_position: &PlayerPosition) -> Vec<Vertex> {
    let x = player_position.coordinates.x;
    let y = player_position.coordinates.y;

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

pub fn get_map_vertices(map: &Map) -> Vec<Vertex> {
    let (x_size, y_size) = map.get_tile_size();

    map.tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, value)| {
                if *value != 0 {
                    Some(get_tile(
                        x as f32,
                        y as f32,
                        x_size,
                        y_size,
                        [1.0, 0.0, 0.0],
                    ))
                } else {
                    Some(get_tile(
                        x as f32,
                        y as f32,
                        x_size,
                        y_size,
                        [0.3, 0.3, 0.3],
                    ))
                }
            })
        })
        .flatten()
        .collect()
}

pub fn get_ray_vertices(player_position: &PlayerPosition, map: &Map) -> Vec<Vertex> {
    let mut rays: Vec<Vertex> = vec![];

    let (tile_size_x, tile_size_y) = map.get_tile_size();
    let ray_start_x = player_position.coordinates.x;
    let ray_start_y = player_position.coordinates.y;
    let mut ray_angle = player_position.angle;
    ray_angle.normalize_as_angle();

    let direction_x = ray_angle.cos();
    let direction_y = ray_angle.sin();

    let current_tile_x = (ray_start_x / tile_size_x).floor();
    let current_tile_y = (ray_start_y / tile_size_y).floor();

    let next_tile_x: f32 = if direction_x > 0.0 {
        (current_tile_x + 1.0) * tile_size_x
    } else {
        current_tile_x * tile_size_x
    };

    let next_tile_y: f32 = if direction_y > 0.0 {
        (current_tile_y + 1.0) * tile_size_y
    } else {
        current_tile_y * tile_size_y
    };

    let mut hit = false;
    while hit {
        hit = true;
    }

    rays.push(Vertex {
        position: [ray_start_x, ray_start_y],
        color: [0.0, 1.0, 0.0],
    });

    rays
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}

fn get_tile(x: f32, y: f32, x_size: f32, y_size: f32, color: [f32; 3]) -> Vec<Vertex> {
    let color = color;

    let x = x * x_size;
    let y = y * y_size;

    let bottom_left = Vertex {
        position: [x + 1.0, y + 1.0],
        color,
    };
    let top_left = Vertex {
        position: [x + 1.0, y + y_size - 1.0],
        color,
    };
    let top_right = Vertex {
        position: [x + x_size - 1.0, y + y_size - 1.0],
        color,
    };
    let bottom_right = Vertex {
        position: [x + x_size - 1.0, y + 1.0],
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
