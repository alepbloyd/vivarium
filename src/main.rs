use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou::{draw, prelude::*};
use std::f64::consts::PI;

const ROWS: u32 = 40;
const COLS: u32 = 40;
const SIZE: u32 = 20;
const LINE_WIDTH: f32 = 1.0;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

trait Nannou {
    fn display(&self, draw: &Draw);
    fn update(&mut self);
}

struct Model {
    random_seed: u64,
    frogs: Vec<Frog>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let random_seed = random_range(0, 1000000);

    let mut frogs: Vec<Frog> = Vec::new();

    let body_point = Point::new(100 as f32, 20 as f32);

    let frog = Frog::new(body_point);
    frogs.push(frog);

    Model { random_seed, frogs }
}

impl Nannou for Model {
    fn display(&self, draw: &Draw) {
        for frog in &self.frogs {
            frog.display(draw);
        }
    }
    fn update(&mut self) {
        for frog in &mut self.frogs {
            frog.update();
        }
    }
}

struct Point {
    x_position: f32,
    y_position: f32,
}

impl Point {
    fn new(x_position: f32, y_position: f32) -> Self {
        Point {
            x_position,
            y_position,
        }
    }
}

struct Frog {
    head_point: Point,
    foot_point: Point,
    body_point: Point,
    mouth_angle: f64,
}

impl Frog {
    fn new(body_point: Point) -> Self {
        let mouth_angle = random_range(30, 60) as f64;
        let base_size = random_range(80, 160) as f64;

        let (head_x, head_y) = get_point_on_circle(
            body_point.x_position as f64,
            body_point.y_position as f64,
            base_size,
            mouth_angle,
        );

        let (foot_x, foot_y) = get_point_on_circle(
            body_point.x_position as f64,
            body_point.y_position as f64,
            base_size,
            0.0,
        );

        let head_point = Point::new(head_x as f32, head_y as f32);
        let foot_point = Point::new(foot_x as f32, foot_y as f32);
        Frog {
            head_point,
            foot_point,
            body_point,
            mouth_angle,
        }
    }
}

impl Nannou for Frog {
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(self.body_point.x_position, self.body_point.y_position)
            .w(10.0)
            .h(10.0);

        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(self.head_point.x_position, self.head_point.y_position)
            .w(10.0)
            .h(10.0);

        draw.ellipse()
            .color(SEAGREEN)
            .stroke(SEAGREEN)
            .stroke_weight(LINE_WIDTH)
            .radius(5.0)
            .x_y(self.foot_point.x_position, self.foot_point.y_position)
            .w(10.0)
            .h(10.0);

        draw.line()
            .start(pt2(self.body_point.x_position, self.body_point.y_position))
            .end(pt2(self.head_point.x_position, self.head_point.y_position))
            .weight(LINE_WIDTH)
            .color(SEAGREEN);

        draw.line()
            .start(pt2(self.body_point.x_position, self.body_point.y_position))
            .end(pt2(self.foot_point.x_position, self.foot_point.y_position))
            .weight(LINE_WIDTH)
            .color(SEAGREEN);
    }
    fn update(&mut self) {
        if self.mouth_angle >= 60.0 {
            self.mouth_angle -= 2.0;
        } else if self.mouth_angle <= 0.0 {
            self.mouth_angle += 2.0;
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // let mut rng = StdRng::seed_from_u64(model.random_seed);
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // Draw model
    model.display(&draw);
    // Render frame
    draw.to_frame(&app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1000000);
        }
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }
}

fn get_point_on_circle(center_x: f64, center_y: f64, radius: f64, angle: f64) -> (f64, f64) {
    let angle_rad = angle * PI / 180.0; // Convert angle to radians

    let x = center_x + radius * f64::cos(angle_rad);

    let y = center_y + radius * f64::sin(angle_rad);

    (x, y)
}
