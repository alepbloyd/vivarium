pub struct Point {
    pub x_position: f64,
    pub y_position: f64,
}

impl Point {
    pub fn new(x_position: f64, y_position: f64) -> Self {
        Point {
            x_position,
            y_position,
        }
    }
}
