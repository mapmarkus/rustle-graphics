use crate::units::{Angle, Distance, Units};

pub struct Draw<T> {
    pub path: T,
}

pub trait Drawable {
    fn move_to(&mut self, x: Units, y: Units);
    fn line_to(&mut self, x: Units, y: Units);
    fn arc(&mut self, x: Units, y: Units, radius: Distance, start_angle: Angle, end_angle: Angle);
}
