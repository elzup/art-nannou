use nannou::{prelude::*};

fn main() {
    nannou::sketch(view_without_model).run();
    nannou::app(model)
        .update(update)
        .simple_window(view)

        .run()
}

struct Model {
    color: Srgb<u8>,
}

fn model(_app: &App) -> Model {
    Model { color: RED }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();

    draw.background().color(_model.color);
    draw.to_frame(_app, &frame).unwrap();

}

fn view_without_model(_app: &App, frame: Frame){
    let draw = _app.draw();

    draw.background().color(BLUE);
    draw.to_frame(_app, &frame).unwrap();

}