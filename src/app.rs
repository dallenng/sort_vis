use std::time::Duration;

use nannou::color::Rgba;
use nannou::event::{Update, WindowEvent};
use nannou::winit::event::VirtualKeyCode;
use nannou::{App, Event, Frame};

use crate::app::array::Array;
use crate::app::sort::bubble::Bubble;
use crate::app::sort::shuffle::Shuffle;

mod array;
mod message;
mod sort;

pub fn run() {
    nannou::app(Model::new).event(event).run();
}

#[derive(Debug, Clone)]
struct Model {
    array: Array,
    active: bool,
    speed: u8,
}

impl Model {
    fn new(app: &App) -> Self {
        app.new_window().title("Sort Vis").maximized(true).view(view).build().unwrap();

        let mut array = Array::new(100);

        array.sort(Shuffle);

        Self { array, active: false, speed: 1 }
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(event), .. } => window_event(app, model, &event),
        Event::Update(up) => update(app, model, &up),
        _ => (),
    }
}

fn window_event(_app: &App, model: &mut Model, event: &WindowEvent) {
    match event {
        WindowEvent::KeyPressed(_) => {}
        WindowEvent::KeyReleased(key) => match key {
            VirtualKeyCode::P => model.active = !model.active,
            VirtualKeyCode::Plus | VirtualKeyCode::NumpadAdd => {
                model.speed = model.speed.saturating_add(1);
            }
            VirtualKeyCode::Minus | VirtualKeyCode::NumpadSubtract => {
                model.speed = model.speed.saturating_sub(1);
            }
            VirtualKeyCode::S => model.array.sort(Bubble),
            VirtualKeyCode::Return => {}
            VirtualKeyCode::Space => model.array.update(1, Duration::from_millis(100)),
            _ => (),
        },
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, update: &Update) {
    if model.active {
        model.array.update(model.speed, update.since_last);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Rgba::new(0.0, 0.0, 0.1, 1.0));

    model.array.view(&draw, app.window_rect());

    draw.to_frame(app, &frame).unwrap();
}
