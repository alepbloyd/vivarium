use nannou::prelude::*;

use crate::entities::point::Point;

pub struct Fly {
    body_point: Point,
}

impl Fly {
    pub fn new(body_point: Point) -> Self {
        Fly { body_point }
    }
}

enum FlyCurrentActions {
    HOVER,
    FLIT,
}

impl crate::Nannou for Fly {
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .color(YELLOW)
            .radius(0.5)
            .x_y(
                self.body_point.x_position as f32,
                self.body_point.y_position as f32,
            )
            .w(5.0)
            .h(5.0);
    }

    fn update(&mut self) {}
}
