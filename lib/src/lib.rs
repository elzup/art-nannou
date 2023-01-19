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

    // Generate sine wave data based on the time of the app
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Clear the background to purple.
    draw.background().color(PURPLE);

    // Draw a blue ellipse at the x/y coordinates 0.0, 0.0
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
