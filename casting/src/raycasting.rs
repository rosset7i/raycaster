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

pub fn get_player_vertex_coordinates(x: f32, y: f32) -> Vec<Vertex> {
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

pub fn get_map_vertex_coordinates(map: &Map) -> Vec<Vertex> {
    let (x_size, y_size) = map.get_tile_size();

    map.tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, value)| {
                if *value != 0 {
                    Some(get_tile(x as f32, y as f32, x_size, y_size))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

pub fn get_ray_vertex_coordinates(player_position: &PlayerPosition, map: &Map) -> Vec<Vertex> {
    let (x_size, y_size) = map.get_tile_size();

    let player_coordinate_y = player_position.coordinates.y;
    let player_coordinate_x = player_position.coordinates.x;
    let mut ray_angle = player_position.angle - 30.0f32.to_radians();
    ray_angle.normalize_as_angle();

    let mut rx: f32 = 0.0;
    let mut ry: f32 = 0.0;
    let mut xo: f32 = 0.0;
    let mut yo: f32 = 0.0;

    let mut rays: Vec<Vertex> = vec![];

    for _i in 0..60 {
        //VERTICAL
        let mut dof = 0;
        let mut dis_h: f32 = 1000000.0;
        let mut hx = player_coordinate_x;
        let mut hy = player_coordinate_y;
        let a_tan: f32 = -1.0 / ray_angle.tan();

        if ray_angle < HALF_CIRCUNFERENCE {
            ry = ((player_coordinate_y / y_size).trunc() * y_size) + y_size + OFFSET;
            rx = (player_coordinate_y - ry) * a_tan + player_coordinate_x;
            yo = y_size;
            xo = -yo * a_tan;
        }

        if ray_angle > HALF_CIRCUNFERENCE {
            ry = ((player_coordinate_y / y_size).trunc() * y_size) - OFFSET;
            rx = (player_coordinate_y - ry) * a_tan + player_coordinate_x;
            yo = -y_size;
            xo = -yo * a_tan;
        }

        if ray_angle == 0.0 || ray_angle == HALF_CIRCUNFERENCE {
            rx = player_coordinate_x;
            ry = player_coordinate_y;
            dof = DEPTH_OF_FIELD;
        }

        while dof < DEPTH_OF_FIELD {
            let mx = (rx / x_size).floor() as u32;
            let my = (ry / y_size).floor() as u32;

            if mx < map.length_x && my < map.length_y && map.tiles[my as usize][mx as usize] == 1 {
                dof = DEPTH_OF_FIELD;
                hx = rx;
                hy = ry;
                dis_h = dist(player_coordinate_x, player_coordinate_y, hx, hy);
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }

        dof = 0;
        let a_tan_neg: f32 = -ray_angle.tan();
        let mut dis_v: f32 = 1000000.0;
        let mut vx = player_coordinate_x;
        let mut vy = player_coordinate_y;
        if (ONE_FORTH_CIRCUNFERENCE..THREE_FORTH_CIRCUNFERENCE).contains(&ray_angle) {
            rx = ((player_coordinate_x / x_size).trunc() * x_size) - OFFSET;
            ry = (player_coordinate_x - rx) * a_tan_neg + player_coordinate_y;
            xo = -x_size;
            yo = -xo * a_tan_neg;
        }

        if !(ONE_FORTH_CIRCUNFERENCE..THREE_FORTH_CIRCUNFERENCE).contains(&ray_angle) {
            rx = ((player_coordinate_x / x_size).trunc() * x_size) + x_size + OFFSET;
            ry = (player_coordinate_x - rx) * a_tan_neg + player_coordinate_y;
            xo = x_size;
            yo = -xo * a_tan_neg;
        }

        if ray_angle == 0.0 || ray_angle == HALF_CIRCUNFERENCE {
            rx = player_coordinate_x;
            ry = player_coordinate_y;
            dof = DEPTH_OF_FIELD;
        }

        while dof < DEPTH_OF_FIELD {
            let mx = (rx / x_size).floor() as u32;
            let my = (ry / y_size).floor() as u32;

            if mx < map.length_x && my < map.length_y && map.tiles[my as usize][mx as usize] == 1 {
                dof = DEPTH_OF_FIELD;

                vx = rx;
                vy = ry;
                dis_v = dist(player_coordinate_x, player_coordinate_y, vx, vy);
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }

        if dis_v < dis_h {
            rx = vx;
            ry = vy;
        }
        if dis_h < dis_v {
            rx = hx;
            ry = hy;
        }
        rays.push(Vertex {
            position: [player_coordinate_x, player_coordinate_y],
            color: [0.0, 1.0, 0.0],
        });
        rays.push(Vertex {
            position: [rx, ry],
            color: [0.0, 1.0, 0.0],
        });

        ray_angle += 1.0f32.to_radians();
        ray_angle.normalize_as_angle();
    }
    rays
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}

fn get_tile(x: f32, y: f32, x_size: f32, y_size: f32) -> Vec<Vertex> {
    let color = [1.0, 0.0, 0.0];

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
