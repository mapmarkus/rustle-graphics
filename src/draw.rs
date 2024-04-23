pub struct Draw<T> {
    pub path: T,
}

pub trait Drawable {
    fn move_to(&mut self, x: f64, y: f64);
    fn line_to(&mut self, x: f64, y: f64);
    fn arc(&mut self, center_x: f64, center_y: f64, radius: f64, start_angle: f64, end_angle: f64);
}
