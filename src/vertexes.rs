use std::f32::consts::PI;

use glium::implement_vertex;

use crate::{
    consts::{EPSILON, MAP, MAP_SIZE, TILE_SIZE},
    movement::PlayerPosition,
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
    let mut dof: u32 = 0;

    let mut rx: f32 = 0.0;
    let mut ry: f32 = 0.0;
    let ra: f32 = player_position.angle;

    let mut xo: f32 = 0.0;
    let mut yo: f32 = 0.0;

    let py: f32 = player_position.coordinates.y;
    let px: f32 = player_position.coordinates.x;
    let mut test: Vec<Vertex> = vec![];
    for _i in 0..=1 {
        dof = 0;
        let mut dis_h: f32 = 1000000.0;
        let mut hx = px;
        let mut hy = py;
        let a_tan: f32 = -1.0 / ra.tan();
        if ra > PI {
            ry = (((py / TILE_SIZE).trunc() as i32) * TILE_SIZE as i32) as f32 - EPSILON;
            rx = (py - ry) * a_tan + px;
            yo = -TILE_SIZE;
            xo = -yo * a_tan;
        }

        if ra < PI {
            ry = (((py / TILE_SIZE).trunc() as i32) * TILE_SIZE as i32) as f32 + TILE_SIZE;
            rx = (py - ry) * a_tan + px;
            yo = TILE_SIZE;
            xo = -yo * a_tan;
        }

        if ra == 0.0 || ra == PI {
            rx = px;
            ry = py;
            dof = 8;
        }

        while dof < 8 {
            let mx = (rx / TILE_SIZE).floor() as u32;
            let my = (ry / TILE_SIZE).floor() as u32;

            if mx < MAP_SIZE && my < MAP_SIZE && MAP[my as usize][mx as usize] == 1 {
                dof = 8;
                hx = rx;
                hy = ry;
                dis_h = dist(px, py, hx, hy, ra);
                //test.push(Vertex {
                //    position: [px, py],
                //    color: [1.0, 1.0, 0.0],
                //});
                //test.push(Vertex {
                //    position: [rx, ry],
                //    color: [1.0, 1.0, 0.0],
                //});
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }

        dof = 0;
        let p2 = PI / 2.0;
        let p3 = 3.0 * PI / 2.0;
        let a_tan: f32 = -ra.tan();
        let mut dis_v: f32 = 1000000.0;
        let mut vx = px;
        let mut vy = py;
        if ra > p2 && ra < p3 {
            rx = (((px / TILE_SIZE).trunc() as i32) * TILE_SIZE as i32) as f32 - EPSILON;
            ry = (px - rx) * a_tan + py;
            xo = -TILE_SIZE;
            yo = -xo * a_tan;
        }

        if ra < p2 || ra > p3 {
            rx = (((px / TILE_SIZE).trunc() as i32) * TILE_SIZE as i32) as f32 + TILE_SIZE;
            ry = (px - rx) * a_tan + py;
            xo = TILE_SIZE;
            yo = -xo * a_tan;
        }

        if ra == 0.0 || ra == PI {
            rx = px;
            ry = py;
            dof = 8;
        }

        while dof < 8 {
            let mx = (rx / TILE_SIZE).floor() as u32;
            let my = (ry / TILE_SIZE).floor() as u32;

            if mx < MAP_SIZE && my < MAP_SIZE && MAP[my as usize][mx as usize] == 1 {
                dof = 8;

                vx = rx;
                vy = ry;
                dis_v = dist(px, py, vx, vy, ra);
                //test.push(Vertex {
                //    position: [px, py],
                //    color: [0.0, 1.0, 0.0],
                //});
                //test.push(Vertex {
                //    position: [rx, ry],
                //    color: [0.0, 1.0, 0.0],
                //});
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
        test.push(Vertex {
            position: [px, py],
            color: [0.0, 1.0, 0.0],
        });
        test.push(Vertex {
            position: [rx, ry],
            color: [0.0, 1.0, 0.0],
        });
    }

    test
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, ang: f32) -> f32 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}
