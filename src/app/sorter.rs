use std::num::NonZeroU8;

use async_executor::{LocalExecutor, Task};
use nannou::event::Update;
use nannou::geom::Rect;
use nannou::Draw;

use crate::app::fixed_time::FixedTime;
use crate::app::sorter::sort::SortAlgorithm;
use crate::app::sorter::view::{SorterView, SorterViewId, SorterViewMap};

mod slice;
pub mod sort;
mod view;

#[derive(Debug)]
pub struct Sorter {
    views: SorterViewMap,
    current_view: SorterViewId,
    executor: LocalExecutor<'static>,
    current_task: Option<Task<()>>,
    running: bool,
    speed: NonZeroU8,
    fixed_time: FixedTime,
}

impl Sorter {
    pub fn new() -> Self {
        Self {
            views: SorterViewMap::default(),
            current_view: SorterViewId::Simple,
            executor: LocalExecutor::new(),
            current_task: None,
            running: true,
            speed: NonZeroU8::MIN,
            fixed_time: FixedTime::default(),
        }
    }

    pub const fn views() -> [SorterViewId; 3] {
        SorterViewId::all()
    }

    pub fn current_speed(&self) -> NonZeroU8 {
        self.speed
    }

    pub fn toggle_running(&mut self) {
        self.running = !self.running;
        self.fixed_time = FixedTime::new(self.fixed_time.period);
    }

    pub fn speed_up(&mut self) {
        self.speed = self.speed.saturating_mul(NonZeroU8::new(2).unwrap());
    }

    pub fn speed_down(&mut self) {
        self.speed = NonZeroU8::new(self.speed.get().saturating_div(2)).unwrap_or(NonZeroU8::MIN);
    }

    pub fn select_view(&mut self, view: SorterViewId) {
        if self.current_view != view {
            self.cancel_current_task();
            self.current_view = view;
        }
    }

    pub fn sort(&mut self, algorithm: SortAlgorithm) {
        self.cancel_current_task();

        let future = algorithm.sort(self.current_view().slice());

        self.current_task = Some(self.executor.spawn(future));
        self.step();
    }

    pub fn step(&mut self) {
        if !self.executor.try_tick() {
            self.current_task = None;
        }
    }

    pub fn update(&mut self, update: Update) {
        if self.running {
            self.fixed_time.tick(update.since_last);

            while self.fixed_time.expend(self.speed).is_ok() {
                self.step();
            }
        }
    }

    pub fn view(&self, draw: &Draw, window_rect: Rect) {
        self.current_view().view(draw, window_rect);
    }

    fn current_view(&self) -> &SorterView {
        &self.views[self.current_view]
    }

    fn cancel_current_task(&mut self) {
        if let Some(task) = self.current_task.take() {
            drop(task);
            self.executor.try_tick();
        }
    }
}
