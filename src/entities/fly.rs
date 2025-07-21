use nannou::prelude::*;

pub struct Fly {
    body_x: f64,
    body_y: f64,
    flight_status: FlightStatus,
}

impl Fly {
    pub fn new(body_x: f64, body_y: f64) -> Self {
        Fly {
            body_x,
            body_y,
            flight_status: FlightStatus::HOVER,
        }
    }
}
#[derive(PartialEq, Eq)]
enum FlightStatus {
    HOVER,
    FLIT,
}

impl crate::Nannou for Fly {
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .color(YELLOW)
            .radius(0.5)
            .x_y(self.body_x as f32, self.body_y as f32)
            .w(5.0)
            .h(5.0);
    }

    fn update(&mut self) {
        // shaky hover
        self.body_x += random_range(-1.0, 1.0);
        self.body_y += random_range(-1.0, 1.0);
    }
}
