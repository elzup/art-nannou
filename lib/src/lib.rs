#![allow(unused)]

use nannou::prelude::*;

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

pub struct Model {
    window: window::Id,
    pub was_updated: bool,
    state: State,
    color: Srgb<u8>,
}

impl Model {
    pub fn for_window(window: window::Id) -> Self {
        Self {
            window,
            state: State::default(),
            was_updated: false,
            color: RED,
        }
    }
}

#[derive(Default, Debug)]
pub struct State {}

#[no_mangle]
pub fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {}

#[no_mangle]
pub fn update(_app: &App, model: &mut Model, _update: Update) {}

#[no_mangle]
pub fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // draw.ellipse().color(STEELBLUE);
    let gdraw = draw.scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    const LINE_WIDTH: f32 = 0.06;
    gdraw.rect()
        .w_h(1.0, 1.0)
        .x_y(0.0, 0.0)
        .stroke(BLACK)
        .stroke_weight(LINE_WIDTH)
        .no_fill();
    for y in 0..ROWS {
        for x in 0..COLS {

            let factor = y as f32 / ROWS as f32;
            let x_offset = factor * random_range(-0.5, 0.5);
            let y_offset = factor * random_range(-0.5, 0.5);
            let rotation = factor * random_range(-PI / 1.0, PI / 1.0);
            let cdraw = gdraw.x_y(x as f32, y as f32);

            cdraw.rect()
                .stroke(BLACK)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(x_offset, y_offset)
                .rotate(rotation)
                .no_fill()
                ;
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
