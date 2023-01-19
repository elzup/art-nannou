#![allow(unused)]

use nannou::prelude::*;

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
            color: GREEN,
        }
    }
}

#[derive(Default, Debug)]
pub struct State {}

#[no_mangle]
pub fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {}

#[no_mangle]
pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

#[no_mangle]
pub fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}
