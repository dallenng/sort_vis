use crate::array::Array;
use crate::sort::bogosort::Bogosort;
use crate::sort::Sort;
use crate::state::{SharedState, State};
use ggez::{Context, GameError};
use std::thread;

const ARRAY_SIZE: u32 = 5;

pub struct App {
    state: SharedState,
    sort_thread: thread::JoinHandle<()>,
}

impl App {
    pub fn new() -> Self {
        let state = SharedState::new(State::new(ARRAY_SIZE));
        let sort_state = state.clone();
        let sort_thread = thread::Builder::new()
            .name(String::from("sort"))
            .spawn(move || {
                let array = Array::new(sort_state);
                array.wait(1000);
                Bogosort::sort(array);
            })
            .unwrap();

        Self { state, sort_thread }
    }
}

impl ggez::event::EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        use ggez::graphics::*;
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        let (window_width, window_height) = drawable_size(ctx);

        let state = self.state.get();
        let array = &state.array;
        let len = array.len() as f32;

        let rect_width = window_width / len;
        let param = DrawParam::default();
        let rect_color = Color::new(1.0, 0.0, 0.0, 1.0);

        for (i, val) in array.iter().enumerate() {
            let rect_height = (*val as f32) * window_height / len;
            let rect = Rect::new(
                (i as f32) * rect_width,
                window_height - rect_height,
                rect_width,
                rect_height,
            );
            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, rect_color).unwrap();
            draw(ctx, &mesh, param).unwrap();
        }

        present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        ggez::graphics::set_screen_coordinates(ctx, [0.0, 0.0, width, height].into()).unwrap();
    }
}
