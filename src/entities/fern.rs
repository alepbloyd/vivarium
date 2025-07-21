use nannou::prelude::*;

use crate::config::config::Config;

pub struct Fern {
    base_x: f64,
    base_y: f64,
    height_cap: f64,
    current_height: f64,
    life_cycle_status: LifeCycleStatus,
    age: i64,
    age_cap: i64,
    green_value: f32,
    red_value: f32,
    blue_value: f32,
}

#[derive(PartialEq, Eq)]

enum LifeCycleStatus {
    JUVENILE,
    ADULT,
    DEAD,
}

impl Fern {
    pub fn new(base_x: f64, base_y: f64) -> Self {
        let height_cap = random_range(100, 200) as f64;
        let age = 0;
        let age_cap = random_range(200, 500);
        let current_height = 0.0;
        let life_cycle_status = LifeCycleStatus::JUVENILE;
        let red_value = random_range(50, 100) as f32;
        let green_value = random_range(200, 255) as f32;
        let blue_value = random_range(50, 100) as f32;
        Fern {
            base_x,
            base_y,
            height_cap,
            current_height,
            life_cycle_status,
            age,
            age_cap,
            red_value,
            green_value,
            blue_value,
        }
    }
}

impl crate::Nannou for Fern {
    fn display(&self, draw: &Draw) {
        let config = Config::new();

        draw.line()
            .start(pt2(self.base_x as f32, 0.0))
            .end(pt2(self.base_x as f32, self.current_height as f32))
            .weight(config.line_width)
            .rgb(self.red_value, self.green_value, self.blue_value);
    }
    fn update(&mut self) {
        self.age = self.age + 1;

        if self.life_cycle_status == LifeCycleStatus::JUVENILE {
            self.current_height += 1.0;
        }

        if self.current_height >= self.height_cap {
            self.life_cycle_status = LifeCycleStatus::ADULT
        }

        if self.age >= self.age_cap {
            self.life_cycle_status = LifeCycleStatus::DEAD;
            self.red_value = 255.0;
            self.green_value = 0.0;
        }
    }
}
