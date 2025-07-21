pub struct Config {
    pub rows: u32,
    pub cols: u32,
    pub size: u32,
    pub line_width: f32,
    pub margin: u32,
    pub width: u32,
    pub height: u32,
    pub debug_mode: bool,
}

impl Config {
    pub fn new() -> Self {
        let rows = 100;
        let cols = 100;
        let size = 10;
        let line_width = 2.0;
        let margin = 35;
        let width = cols * size + 2 * margin;
        let height = rows * size + 2 * margin;
        let debug_mode = false;
        Config {
            rows,
            cols,
            size,
            line_width,
            margin,
            width,
            height,
            debug_mode,
        }
    }
}
