use nannou::prelude::*;
use nannou::rand::distributions::Standard;
use nannou::rand::prelude::Distribution;
use nannou::rand::Rng;

use crate::entities::point::Point;

use crate::geometry_utils::circle_math::*;

use crate::LINE_WIDTH;

#[derive(PartialEq, Eq)]

enum MouthDirections {
    OPENING,
    CLOSING,
    STATIC,
}

pub struct Frog {
    head_point: Point,
    foot_point: Point,
    body_point: Point,
    mouth_angle: f64,
    mouth_direction: MouthDirections,
    size: f64,
}

impl Distribution<MouthDirections> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> MouthDirections {
        match random_range(0, 2) {
            0 => MouthDirections::OPENING,
            1 => MouthDirections::CLOSING,
            _ => MouthDirections::STATIC,
        }
    }
}

impl Frog {
    // need to keep track of if mouth is 'opening' or 'closing' for animating
    pub fn new(body_point: Point) -> Self {
        let mouth_angle = random_range(30, 60) as f64;
        let size = random_range(80, 160) as f64;
        let mouth_direction: MouthDirections = nannou::rand::random();

        let (head_x, head_y) = get_point_on_circle(
            body_point.x_position as f64,
            body_point.y_position as f64,
            size,
            mouth_angle,
        );

        let (foot_x, foot_y) = get_point_on_circle(
            body_point.x_position as f64,
            body_point.y_position as f64,
            size,
            0.0,
        );

        let head_point = Point::new(head_x as f64, head_y as f64);
        let foot_point = Point::new(foot_x as f64, foot_y as f64);
        Frog {
            head_point,
            foot_point,
            body_point,
            mouth_angle,
            mouth_direction,
            size,
        }
    }
}

impl crate::Nannou for Frog {
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(
                self.body_point.x_position as f32,
                self.body_point.y_position as f32,
            )
            .w(10.0)
            .h(10.0);

        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(
                self.head_point.x_position as f32,
                self.head_point.y_position as f32,
            )
            .w(10.0)
            .h(10.0);

        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(
                self.foot_point.x_position as f32,
                self.foot_point.y_position as f32,
            )
            .w(10.0)
            .h(10.0);

        draw.line()
            .start(pt2(
                self.body_point.x_position as f32,
                self.body_point.y_position as f32,
            ))
            .end(pt2(
                self.head_point.x_position as f32,
                self.head_point.y_position as f32,
            ))
            .weight(LINE_WIDTH)
            .color(SEAGREEN);

        draw.line()
            .start(pt2(
                self.body_point.x_position as f32,
                self.body_point.y_position as f32,
            ))
            .end(pt2(
                self.foot_point.x_position as f32,
                self.foot_point.y_position as f32,
            ))
            .weight(LINE_WIDTH)
            .color(SEAGREEN);
    }
    fn update(&mut self) {
        if self.body_point.y_position > 1.0 {
            self.body_point.y_position -= 1.0;
            self.foot_point.y_position -= 1.0;
            self.head_point.y_position -= 1.0;
        }

        // mouth open/close
        if self.mouth_direction == MouthDirections::OPENING && self.mouth_angle <= 40.0 {
            self.mouth_angle += 0.20;
            let (new_x, new_y) = get_point_on_circle(
                self.body_point.x_position as f64,
                self.body_point.y_position as f64,
                self.size,
                self.mouth_angle,
            );
            self.head_point.x_position = new_x;
            self.head_point.y_position = new_y;
        } else if self.mouth_direction == MouthDirections::OPENING && self.mouth_angle > 40.0 {
            self.mouth_direction = MouthDirections::CLOSING;
        };

        if self.mouth_direction == MouthDirections::CLOSING && self.mouth_angle >= 0.0 {
            self.mouth_angle -= 0.20;
            let (new_x, new_y) = get_point_on_circle(
                self.body_point.x_position as f64,
                self.body_point.y_position as f64,
                self.size,
                self.mouth_angle,
            );
            self.head_point.x_position = new_x;
            self.head_point.y_position = new_y;
        } else if self.mouth_direction == MouthDirections::CLOSING && self.mouth_angle < 0.0 {
            self.mouth_direction = MouthDirections::OPENING;
        };

        if self.mouth_direction == MouthDirections::STATIC {
            self.mouth_direction = MouthDirections::OPENING
        }

        // self.body_point.x_position -= 2.0;
    }
}
