use std::iter;

use nannou::draw::primitive;
use nannou::geom::Rect;
use nannou::{color, Draw};

use crate::app::sorter::slice::SharedSlice;
use crate::app::sorter::view::View;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleView {
    slice: SharedSlice,
    len: f32,
}

impl View for SimpleView {
    fn slice(&self) -> SharedSlice {
        self.slice.clone()
    }

    fn view(&self, draw: &Draw, window_rect: Rect) {
        let len = self.len;

        let rect_w = window_rect.w() / len;
        let rect_h_factor = window_rect.h() / len;

        for (item, x) in
            self.slice.inner().iter().zip(iter::successors(Some(0.0), |x| Some(x + rect_w)))
        {
            let value = f32::from(item.value());
            let access_value = (1.0 - item.last_access_elapsed().as_secs_f32() * 2.0).max(0.0);

            let rect = Rect::from_w_h(rect_w, rect_h_factor * value)
                .bottom_left_of(window_rect)
                .shift_x(x);
            let rect = primitive::Rect::from(rect);

            draw.a(rect.clone()).color(color::srgba((value / len).clamp(0.2, 1.0), 0.2, 0.9, 1.0));
            draw.a(rect).color(color::srgba(access_value, access_value, access_value, 0.4));
        }
    }
}

impl SimpleView {
    pub fn new(len: u16) -> Self {
        Self { slice: (1..=len).collect(), len: f32::from(len) }
    }
}
