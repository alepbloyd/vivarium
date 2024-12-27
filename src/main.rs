use nannou::prelude::*;
use nannou::rand::distributions::Standard;
use nannou::rand::prelude::Distribution;
use nannou::rand::Rng;
use std::f64::consts::PI;

const ROWS: u32 = 100;
const COLS: u32 = 100;
const SIZE: u32 = 10;
const LINE_WIDTH: f32 = 2.0;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
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

trait Nannou {
    fn display(&self, draw: &Draw);
    fn update(&mut self);
}

struct Model {
    random_seed: u64,
    frogs: Vec<Frog>,
    flies: Vec<Fly>,
}

fn model(app: &App) -> Model {
    // let 🦀 = 1;
    // let 🐈 = 1;
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
    let mut flies: Vec<Fly> = Vec::new();

    for _n in 1..6 {
        let x = random_range(0, 800);
        let y = random_range(0, 800);
        let body_point = Point::new(x as f64, y as f64);
        let frog = Frog::new(body_point);
        frogs.push(frog);
    }

    for _n in 1..20 {
        let x = random_range(0, 800);
        let y = random_range(0, 800);
        let body_point = Point::new(x as f64, y as f64);
        let fly = Fly::new(body_point);
        flies.push(fly);
    }

    Model {
        random_seed,
        frogs,
        flies,
    }
}

impl Nannou for Model {
    fn display(&self, draw: &Draw) {
        for frog in &self.frogs {
            frog.display(draw);
        }
        for fly in &self.flies {
            fly.display(draw);
        }
    }
    fn update(&mut self) {
        for frog in &mut self.frogs {
            frog.update();
        }
        for fly in &mut self.flies {
            fly.update();
        }
    }
}

struct Point {
    x_position: f64,
    y_position: f64,
}

impl Point {
    fn new(x_position: f64, y_position: f64) -> Self {
        Point {
            x_position,
            y_position,
        }
    }
}

#[derive(PartialEq, Eq)]

enum MouthDirections {
    OPENING,
    CLOSING,
    STATIC,
}

struct Frog {
    head_point: Point,
    foot_point: Point,
    body_point: Point,
    mouth_angle: f64,
    mouth_direction: MouthDirections,
    size: f64,
}

impl Frog {
    // need to keep track of if mouth is 'opening' or 'closing' for animating
    fn new(body_point: Point) -> Self {
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

impl Nannou for Frog {
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

struct Fly {
    body_point: Point,
}

impl Fly {
    fn new(body_point: Point) -> Self {
        Fly { body_point }
    }
}

enum FlyCurrentActions {
    HOVER,
    FLIT,
}

impl Nannou for Fly {
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

fn update(_app: &App, model: &mut Model, _update: Update) {
    // let mut rng = StdRng::seed_from_u64(model.random_seed);
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().x_y(-400.0, -400.0);

    draw.background().color(BLACK);

    draw.line()
        .start(pt2(-1000.0, 0.0))
        .end(pt2(1000.0, 0.0))
        .color(RED);

    draw.line()
        .start(pt2(0.0, -1000.0))
        .end(pt2(0.0, 1000.0))
        .color(RED);

    // y-axis ticks
    for n in 0..100 {
        draw.line()
            .start(pt2(-10.0, n as f32 * 10.0))
            .end(pt2(10.0, n as f32 * 10.0))
            .color(WHITE);
        if n % 5 == 0 {
            draw.line()
                .start(pt2(-10.0, n as f32 * 10.0))
                .end(pt2(10.0, n as f32 * 10.0))
                .color(MAGENTA);
        }
    }

    // x-axis ticks
    for n in 0..100 {
        draw.line()
            .start(pt2(n as f32 * 10.0, 10.0))
            .end(pt2(n as f32 * 10.0, -10.0))
            .color(WHITE);
        if n % 5 == 0 {
            draw.line()
                .start(pt2(n as f32 * 10.0, -10.0))
                .end(pt2(n as f32 * 10.0, 10.0))
                .color(MAGENTA);
        }
    }

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
