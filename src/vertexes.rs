use glium::implement_vertex;

use crate::{
    consts::{
        EPSILON, HALF_CIRCUNFERENCE, MAP, MAP_SIZE, ONE_FORTH_CIRCUNFERENCE,
        THREE_FORTH_CIRCUNFERENCE, TILE_SIZE,
    },
    movement::{Angle, PlayerPosition},
};

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

pub fn draw_rays(player_position: &PlayerPosition) -> Vec<Vertex> {
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
        let mut dof = 0;
        let mut dis_h: f32 = 1000000.0;
        let mut hx = player_coordinate_x;
        let mut hy = player_coordinate_y;
        let a_tan: f32 = -1.0 / ray_angle.tan();

        if ray_angle < HALF_CIRCUNFERENCE {
            ry = (((player_coordinate_y / TILE_SIZE).trunc() as u32) * TILE_SIZE as u32) as f32
                + TILE_SIZE;
            rx = (player_coordinate_y - ry) * a_tan + player_coordinate_x;
            yo = TILE_SIZE;
            xo = -yo * a_tan;
        }

        if ray_angle > HALF_CIRCUNFERENCE {
            ry = (((player_coordinate_y / TILE_SIZE).trunc() as u32) * TILE_SIZE as u32) as f32
                - EPSILON;
            rx = (player_coordinate_y - ry) * a_tan + player_coordinate_x;
            yo = -TILE_SIZE;
            xo = -yo * a_tan;
        }

        if ray_angle == 0.0 || ray_angle == HALF_CIRCUNFERENCE {
            rx = player_coordinate_x;
            ry = player_coordinate_y;
            dof = 8;
        }

        while dof < 8 {
            let mx = (rx / TILE_SIZE).floor() as u32;
            let my = (ry / TILE_SIZE).floor() as u32;

            if mx < MAP_SIZE && my < MAP_SIZE && MAP[my as usize][mx as usize] == 1 {
                dof = 8;
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
            rx = (((player_coordinate_x / TILE_SIZE).trunc() as u32) * TILE_SIZE as u32) as f32
                - EPSILON;
            ry = (player_coordinate_x - rx) * a_tan_neg + player_coordinate_y;
            xo = -TILE_SIZE;
            yo = -xo * a_tan_neg;
        }

        if !(ONE_FORTH_CIRCUNFERENCE..THREE_FORTH_CIRCUNFERENCE).contains(&ray_angle) {
            rx = (((player_coordinate_x / TILE_SIZE).trunc() as u32) * TILE_SIZE as u32) as f32
                + TILE_SIZE;
            ry = (player_coordinate_x - rx) * a_tan_neg + player_coordinate_y;
            xo = TILE_SIZE;
            yo = -xo * a_tan_neg;
        }

        if ray_angle == 0.0 || ray_angle == HALF_CIRCUNFERENCE {
            rx = player_coordinate_x;
            ry = player_coordinate_y;
            dof = 8;
        }

        while dof < 8 {
            let mx = (rx / TILE_SIZE).floor() as u32;
            let my = (ry / TILE_SIZE).floor() as u32;

            if mx < MAP_SIZE && my < MAP_SIZE && MAP[my as usize][mx as usize] == 1 {
                dof = 8;

                vx = rx;
                vy = ry;
                dis_v = dist(player_coordinate_x, player_coordinate_y, vx, vy);
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }

        //println!(
        //    "__________________\nPX: {player_coordinate_x}\nPY: {player_coordinate_y}\nRA: {ray_angle}\nRX: {rx}\nRY: {ry}\nXO: {xo}\nYO: {yo}\nHX: {hx}\nHY: {hy}\nATAN: {a_tan}\nATANNEG: {a_tan_neg}"
        //);
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
