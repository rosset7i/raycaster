use std::f32::consts::{FRAC_PI_2, PI};

use crate::{
    map::Map,
    math::Angle,
    player::{PlayerPosition, Point},
};

pub const OFFSET: f32 = 0.0001;
pub const DEPTH_OF_FIELD: u32 = 10;
pub const FIELD_OF_VISION: usize = 60;

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
    let (tile_height, tile_width) = map.get_tile_size();

    map.tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, value)| {
                if *value != 0 {
                    get_tile(x as f32, y as f32, tile_height, tile_width, [1.0, 0.0, 0.0])
                } else {
                    get_tile(x as f32, y as f32, tile_height, tile_width, [0.3, 0.3, 0.3])
                }
            })
        })
        .flatten()
        .collect()
}

pub fn get_ray_vertices(player_position: &PlayerPosition, map: &Map) -> Vec<Vertex> {
    let (tile_width, tile_height) = map.get_tile_size();
    let mut ray_angle = (player_position.angle - 30.0f32.to_radians()).normalize_as_radians();

    let mut rays = Vec::with_capacity(FIELD_OF_VISION * 2);

    for _i in 0..FIELD_OF_VISION {
        let (horizontal_hit, horizontal_dist) = cast_ray(
            &player_position.coordinates,
            ray_angle,
            tile_width,
            tile_height,
            map,
            true,
        );

        let (vertical_hit, vertical_dist) = cast_ray(
            &player_position.coordinates,
            ray_angle,
            tile_width,
            tile_height,
            map,
            false,
        );

        let hit_point = if horizontal_dist < vertical_dist {
            horizontal_hit
        } else {
            vertical_hit
        };

        rays.push(Vertex {
            position: [player_position.coordinates.x, player_position.coordinates.y],
            color: [0.0, 1.0, 0.0],
        });
        rays.push(Vertex {
            position: [hit_point.x, hit_point.y],
            color: [0.0, 1.0, 0.0],
        });

        ray_angle = (ray_angle + 1.0f32.to_radians()).normalize_as_radians();
    }
    rays
}

fn cast_ray(
    ray_origin: &Point,
    angle: f32,
    tile_width: f32,
    tile_heigth: f32,
    map: &Map,
    horizontal: bool,
) -> (Point, f32) {
    let (mut ray_x, mut ray_y, step_x, step_y): (f32, f32, f32, f32);
    let mut depth = 0;
    let mut hit = ray_origin.clone();

    if horizontal {
        let atan = -1.0 / angle.tan();
        if angle < PI {
            ray_y = (ray_origin.y / tile_heigth).floor() * tile_heigth + tile_heigth + OFFSET;
            ray_x = (ray_origin.y - ray_y) * atan + ray_origin.x;
            step_y = tile_heigth;
            step_x = -step_y * atan;
        } else if angle > PI {
            ray_y = (ray_origin.y / tile_heigth).floor() * tile_heigth - OFFSET;
            ray_x = (ray_origin.y - ray_y) * atan + ray_origin.x;
            step_y = -tile_heigth;
            step_x = -step_y * atan;
        } else {
            return (hit, f32::MAX);
        }
    } else {
        let ntan = -angle.tan();
        if (FRAC_PI_2..FRAC_PI_2 * 3.0).contains(&angle) {
            ray_x = (ray_origin.x / tile_width).floor() * tile_width - OFFSET;
            ray_y = (ray_origin.x - ray_x) * ntan + ray_origin.y;
            step_x = -tile_width;
            step_y = -step_x * ntan;
        } else {
            ray_x = (ray_origin.x / tile_width).floor() * tile_width + tile_width + OFFSET;
            ray_y = (ray_origin.x - ray_x) * ntan + ray_origin.y;
            step_x = tile_width;
            step_y = -step_x * ntan;
        }
    }

    while depth < DEPTH_OF_FIELD {
        let map_x = (ray_x / tile_width).floor() as usize;
        let map_y = (ray_y / tile_heigth).floor() as usize;

        if map_x < map.length_x as usize
            && map_y < map.length_y as usize
            && map.tiles[map_y][map_x] == 1
        {
            hit = Point { x: ray_x, y: ray_y };
            let distance = get_distance(ray_origin, ray_x, ray_y);
            return (hit, distance);
        }

        ray_x += step_x;
        ray_y += step_y;
        depth += 1;
    }

    (hit, f32::MAX)
}

fn get_distance(ray_origin: &Point, ray_x: f32, ray_y: f32) -> f32 {
    ((ray_x - ray_origin.x).powi(2) + (ray_y - ray_origin.y).powi(2)).sqrt()
}

fn get_tile(x: f32, y: f32, x_size: f32, y_size: f32, color: [f32; 3]) -> Vec<Vertex> {
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
