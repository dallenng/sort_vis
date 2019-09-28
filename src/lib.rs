use crate::app::App;
use crate::array::Array;
use crate::sort::all_sort_functions;
use crate::sort::bubble::bubble_sort;
use ggez::conf::WindowMode;
use std::env::Args;

mod app;
mod array;
mod sort;
mod state;

pub struct Config {
    pub sort_function: fn(Array),
}

impl Config {
    pub fn new(mut args: Args) -> Result<Self, &'static str> {
        args.next();
        let sort_function = match args.next() {
            Some(arg) => match all_sort_functions().get(arg.as_str()) {
                Some(sort) => *sort,
                None => bubble_sort,
            },
            None => bubble_sort,
        };

        Ok(Self { sort_function })
    }
}

pub fn run(config: Config) -> ggez::GameResult {
    let mut mode: WindowMode = Default::default();
    mode.maximized = true;
    mode.resizable = true;
    let cb = ggez::ContextBuilder::new("sort_vis", "dallenng").window_mode(mode);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut App::new(config.sort_function);
    ggez::event::run(ctx, event_loop, state)
}
