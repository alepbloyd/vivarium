use nannou::prelude::*;
use nannou::rand::distributions::Standard;
use nannou::rand::prelude::Distribution;
use nannou::rand::Rng;

use crate::geometry_utils::circle_math::*;

use crate::LINE_WIDTH;

#[derive(PartialEq, Eq)]

enum MouthStatus {
    OPENING,
    CLOSING,
    STATIC,
}

pub struct Frog {
    head_x: f64,
    head_y: f64,
    foot_x: f64,
    foot_y: f64,
    body_x: f64,
    body_y: f64,
    mouth_angle: f64,
    mouth_status: MouthStatus,
    size: f64,
}

impl Distribution<MouthStatus> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> MouthStatus {
        match random_range(0, 2) {
            0 => MouthStatus::OPENING,
            1 => MouthStatus::CLOSING,
            _ => MouthStatus::STATIC,
        }
    }
}

impl Frog {
    // need to keep track of if mouth is 'opening' or 'closing' for animating
    pub fn new(body_x: f64, body_y: f64) -> Self {
        let mouth_angle = random_range(30, 60) as f64;
        let size = random_range(80, 160) as f64;
        let mouth_status: MouthStatus = nannou::rand::random();

        let (head_x, head_y) = get_point_on_circle(body_x, body_y, size, mouth_angle);

        let (foot_x, foot_y) = get_point_on_circle(body_x, body_y, size, 0.0);

        Frog {
            head_x,
            head_y,
            foot_x,
            foot_y,
            body_x,
            body_y,
            mouth_angle,
            mouth_status,
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
            .x_y(self.body_x as f32, self.body_y as f32)
            .w(10.0)
            .h(10.0);

        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(self.head_x as f32, self.head_y as f32)
            .w(10.0)
            .h(10.0);

        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(self.foot_x as f32, self.foot_y as f32)
            .w(10.0)
            .h(10.0);

        draw.line()
            .start(pt2(self.body_x as f32, self.body_y as f32))
            .end(pt2(self.head_x as f32, self.head_y as f32))
            .weight(LINE_WIDTH)
            .color(SEAGREEN);

        draw.line()
            .start(pt2(self.body_x as f32, self.body_y as f32))
            .end(pt2(self.foot_x as f32, self.foot_y as f32))
            .weight(LINE_WIDTH)
            .color(SEAGREEN);
    }
    fn update(&mut self) {
        if self.body_y > 1.0 {
            self.body_y -= 1.0;
            self.foot_y -= 1.0;
            self.head_y -= 1.0;
        }

        // mouth open/close
        if self.mouth_status == MouthStatus::OPENING && self.mouth_angle <= 40.0 {
            self.mouth_angle += 0.20;
            let (new_x, new_y) = get_point_on_circle(
                self.body_x as f64,
                self.body_y as f64,
                self.size,
                self.mouth_angle,
            );
            self.head_x = new_x;
            self.head_y = new_y;
        } else if self.mouth_status == MouthStatus::OPENING && self.mouth_angle > 40.0 {
            self.mouth_status = MouthStatus::CLOSING;
        };

        if self.mouth_status == MouthStatus::CLOSING && self.mouth_angle >= 0.0 {
            self.mouth_angle -= 0.20;
            let (new_x, new_y) = get_point_on_circle(
                self.body_x as f64,
                self.body_y as f64,
                self.size,
                self.mouth_angle,
            );
            self.head_x = new_x;
            self.head_y = new_y;
        } else if self.mouth_status == MouthStatus::CLOSING && self.mouth_angle < 0.0 {
            self.mouth_status = MouthStatus::OPENING;
        };

        if self.mouth_status == MouthStatus::STATIC {
            self.mouth_status = MouthStatus::OPENING
        }

        // self.body_point.x_position -= 2.0;
    }
}
