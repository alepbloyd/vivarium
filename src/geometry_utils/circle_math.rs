use std::f64::consts::PI;

pub fn get_point_on_circle(center_x: f64, center_y: f64, radius: f64, angle: f64) -> (f64, f64) {
    let angle_rad = angle * PI / 180.0; // Convert angle to radians
    let x = center_x + radius * f64::cos(angle_rad);
    let y = center_y + radius * f64::sin(angle_rad);
    (x, y)
}
