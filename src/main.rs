use nannou::prelude::*;

mod entities;
use crate::entities::fern::Fern;
use crate::entities::fly::Fly;
use crate::entities::frog::Frog;

mod config;
use crate::config::config::Config; // what the hell is this, no.
mod geometry_utils;

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
    flies: Vec<Fly>,
    ferns: Vec<Fern>,
}

fn model(app: &App) -> Model {
    // let 🦀 = 1;
    // let 🐈 = 1;
    let config = Config::new();
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(config.width, config.height)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let random_seed = random_range(0, 1000000);

    let mut frogs: Vec<Frog> = Vec::new();
    let mut flies: Vec<Fly> = Vec::new();
    let mut ferns: Vec<Fern> = Vec::new();

    for _n in 1..6 {
        let x = random_range(0, 800);
        let y = random_range(0, 500);
        let frog = Frog::new(x as f64, y as f64);
        frogs.push(frog);
    }

    for _n in 1..100 {
        let x = random_range(0, 800);
        let y = random_range(0, 800);
        let fly = Fly::new(x as f64, y as f64);
        flies.push(fly);
    }

    for _n in 1..10 {
        let x = random_range(0, 1000);
        let fern = Fern::new(x as f64, 0.0);
        ferns.push(fern)
    }

    Model {
        random_seed,
        frogs,
        flies,
        ferns,
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
        for fern in &self.ferns {
            fern.display(draw);
        }
    }
    fn update(&mut self) {
        for frog in &mut self.frogs {
            frog.update();
        }
        for fly in &mut self.flies {
            fly.update();
        }
        for fern in &mut self.ferns {
            fern.update();
        }
    }
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
