use crate::math::Angle;

pub const VELOCITY_MULTIPLIER: f32 = 2.5;
pub const SENSITIVITY_MULTIPLIER: f32 = 0.03;

#[derive(Default, Clone)]
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
            Direction::Left => self.angle += SENSITIVITY_MULTIPLIER,
            Direction::Right => self.angle -= SENSITIVITY_MULTIPLIER,
        }

        self.angle.normalize_as_radians();
    }
}
