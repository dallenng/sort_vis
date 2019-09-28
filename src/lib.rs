use crate::app::App;
use ggez::conf::WindowMode;
use std::env::Args;

mod app;
mod array;
mod sort;
mod state;

pub struct Config {}

impl Config {
    pub fn new(_args: Args) -> Result<Self, &'static str> {
        Ok(Self {})
    }
}

pub fn run(_config: Config) -> ggez::GameResult {
    let mut mode: WindowMode = Default::default();
    mode.maximized = true;
    mode.resizable = true;
    let cb = ggez::ContextBuilder::new("sort_vis", "dallenng").window_mode(mode);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut App::new();
    ggez::event::run(ctx, event_loop, state)
}
