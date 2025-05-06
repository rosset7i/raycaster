use std::f32::consts::PI;

const COMPLETE_CIRCUNFERENCE: f32 = PI * 2.0;

pub trait Angle {
    fn normalize_as_radians(&mut self) -> Self;
}

impl Angle for f32 {
    fn normalize_as_radians(&mut self) -> Self {
        if *self < 0.0 {
            *self += COMPLETE_CIRCUNFERENCE;
        } else if *self > COMPLETE_CIRCUNFERENCE {
            *self -= COMPLETE_CIRCUNFERENCE;
        }

        *self
    }
}
