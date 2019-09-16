use ggez::conf::{FullscreenType, WindowMode};
use std::env::Args;

mod state;

pub struct Config {}

impl Config {
    pub fn new(_args: Args) -> Result<Self, &'static str> {
        Ok(Self {})
    }
}

pub fn run(_config: Config) -> ggez::GameResult {
    let mode = WindowMode {
        width: 1280.0,
        height: 720.0,
        maximized: true,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        min_height: 0.0,
        max_width: 0.0,
        max_height: 0.0,
        resizable: true,
    };
    let cb = ggez::ContextBuilder::new("sort_vis", "dallenng").window_mode(mode);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut state::IterableSortVec::new(1_000);
    ggez::event::run(ctx, event_loop, state)
}
