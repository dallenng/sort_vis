use crate::array::Array;
use crate::state::{SharedState, State};
use ggez::graphics::{Color, DrawParam, Rect};
use ggez::{Context, GameError};
use std::thread;
use std::time::Duration;

const CLEAR_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);
const RECTANGLE_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);

pub struct App {
    state: SharedState,
    rectangle: Rect,
    param: DrawParam,
    sort_thread: thread::JoinHandle<()>,
}

impl App {
    pub fn new(sort: fn(Array), size: usize) -> Self {
        let state = SharedState::new(State::new(size));
        let sort_state = state.clone();
        let sort_thread = thread::Builder::new()
            .name(String::from("sort"))
            .spawn(move || {
                let array = Array::new(sort_state);
                array.wait(Duration::from_secs(1));
                sort(array);
            })
            .unwrap();

        let rectangle = Rect::new_i32(0, 0, 1, 1);
        let param = DrawParam::default();

        Self {
            state,
            sort_thread,
            rectangle,
            param,
        }
    }
}

impl ggez::event::EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        use ggez::graphics::*;
        clear(ctx, CLEAR_COLOR);

        let (window_width, window_height) = drawable_size(ctx);

        let state = self.state.get();
        let array = &state.array;
        let len = array.len() as f32;

        let rect_width = window_width / len;
        let mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), self.rectangle, RECTANGLE_COLOR).unwrap();

        self.param.dest.x = 0.0;
        self.param.scale.x = rect_width;

        for val in array {
            let rect_height = *val * window_height;
            self.param.dest.y = window_height - rect_height;
            self.param.scale.y = rect_height;

            draw(ctx, &mesh, self.param).unwrap();

            self.param.dest.x += rect_width;
        }

        present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        ggez::graphics::set_screen_coordinates(ctx, [0.0, 0.0, width, height].into()).unwrap();
    }
}
