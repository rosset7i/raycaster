use std::f32::consts::PI;

pub const COMPLETE_CIRCUNFERENCE: f32 = PI * 2.0;
pub const START_CIRCUNFERENCE: f32 = 0.0;
pub const ONE_FORTH_CIRCUNFERENCE: f32 = PI / 2.0;
pub const HALF_CIRCUNFERENCE: f32 = PI;
pub const THREE_FORTH_CIRCUNFERENCE: f32 = 3.0 * PI / 2.0;

pub trait Angle {
    fn normalize_as_angle(&mut self);
}

impl Angle for f32 {
    fn normalize_as_angle(&mut self) {
        if *self < 0.0 {
            *self += COMPLETE_CIRCUNFERENCE;
        } else if *self > COMPLETE_CIRCUNFERENCE {
            *self -= COMPLETE_CIRCUNFERENCE;
        }
    }
}
