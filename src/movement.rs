use std::f32::consts::PI;

use crate::consts::VELOCITY_MULTIPLIER;

#[derive(Default, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn from(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

pub enum Direction {
    Left,
    Right,
}

#[derive(Default)]
pub struct PlayerPosition {
    pub coordinates: Point,
    pub angle: f32,
}

impl PlayerPosition {
    pub fn new(coordinates: Point, angle: f32) -> PlayerPosition {
        PlayerPosition { coordinates, angle }
    }

    fn get_deltas(&self) -> Point {
        Point {
            x: self.angle.cos() * VELOCITY_MULTIPLIER,
            y: self.angle.sin() * VELOCITY_MULTIPLIER,
        }
    }

    pub fn move_up(&mut self) {
        let deltas = self.get_deltas();

        self.coordinates.x += deltas.x;
        self.coordinates.y += deltas.y;
    }

    pub fn move_down(&mut self) {
        let deltas = self.get_deltas();

        self.coordinates.x -= deltas.x;
        self.coordinates.y -= deltas.y;
    }

    pub fn rotate(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.angle -= 0.1,
            Direction::Right => self.angle += 0.1,
        }

        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        } else if self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }

    pub fn get_camera_line_position(&self) -> Point {
        let deltas = self.get_deltas();

        Point {
            x: self.coordinates.x + deltas.x * VELOCITY_MULTIPLIER,
            y: self.coordinates.y + deltas.y * VELOCITY_MULTIPLIER,
        }
    }
}
