use nannou::event::{Update, WindowEvent};
use nannou::winit::event::VirtualKeyCode;
use nannou::{color, App, Event, Frame};
use nannou_egui::egui::{Align2, TextStyle};
use nannou_egui::{egui, Egui};

use crate::app::sorter::sort::SortAlgorithm;
use crate::app::sorter::Sorter;

mod fixed_time;
mod sorter;

pub fn run() {
    nannou::app(Model::new).event(event).run();
}

struct Model {
    sorter: Sorter,
    egui: Egui,
    show_sort_menu: bool,
    show_view_menu: bool,
    font_scaled: bool,
}

impl Model {
    fn new(app: &App) -> Self {
        let sorter = Sorter::new();

        let window = app
            .new_window()
            .view(view)
            .raw_event(|_, model: &mut Self, event| model.egui.handle_raw_event(event))
            .title("Sort visualizer")
            .maximized(true)
            .build()
            .unwrap();

        let egui = Egui::from_window(&app.window(window).unwrap());

        Model { sorter, egui, show_sort_menu: false, show_view_menu: false, font_scaled: false }
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(event), .. } => window_event(app, model, event),
        Event::Update(up) => update(app, model, up),
        _ => (),
    }
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    if let WindowEvent::KeyPressed(key) = event {
        match key {
            VirtualKeyCode::S => model.show_sort_menu = !model.show_sort_menu,
            VirtualKeyCode::V => model.show_view_menu = !model.show_view_menu,
            VirtualKeyCode::P => model.sorter.toggle_running(),
            VirtualKeyCode::Space => model.sorter.step(),
            VirtualKeyCode::Plus | VirtualKeyCode::NumpadAdd => model.sorter.speed_up(),
            VirtualKeyCode::Minus | VirtualKeyCode::NumpadSubtract => model.sorter.speed_down(),
            _ => (),
        }
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.sorter.update(update);

    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    if !model.font_scaled {
        let mut fonts = ctx.fonts().definitions().clone();
        fonts.family_and_size.get_mut(&TextStyle::Body).unwrap().1 *= 1.5;

        ctx.set_fonts(fonts);
        model.font_scaled = true;
    }

    egui::Window::new("Menu").resizable(false).show(&ctx, |ui| {
        ui.label("(S) Sort menu");
        ui.label("(V) View menu");
        ui.label("(P) Pause/Resume");
        ui.label("(Space) Make a step");
        ui.label(format!("(+/-) Speed up/down (x{})", model.sorter.current_speed()));
    });

    if model.show_sort_menu {
        egui::Window::new("Sort menu")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .resizable(false)
            .collapsible(false)
            .show(&ctx, |ui| {
                for algorithm in SortAlgorithm::all() {
                    if ui.button(algorithm).clicked() {
                        model.sorter.sort(algorithm);
                        model.show_sort_menu = false;
                    }
                }
            });
    }

    if model.show_view_menu {
        egui::Window::new("View menu")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .resizable(false)
            .collapsible(false)
            .show(&ctx, |ui| {
                for view in Sorter::views() {
                    if ui.button(view).clicked() {
                        model.sorter.select_view(view);
                        model.show_view_menu = false;
                    }
                }
            });
    }
}

fn view(app: &App, model: &Model, frame: Frame<'_>) {
    let draw = app.draw();
    draw.background().color(color::BLACK);

    model.sorter.view(&draw, app.window_rect());

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
