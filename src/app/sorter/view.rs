use std::fmt;
use std::ops::Index;

use nannou::geom::Rect;
use nannou::Draw;

use crate::app::sorter::slice::SharedSlice;
use crate::app::sorter::view::simple::SimpleView;

mod simple;

trait View {
    fn slice(&self) -> SharedSlice;

    fn view(&self, draw: &Draw, window_rect: Rect);
}

#[derive(Debug)]
pub struct SorterViewMap {
    inner: [SorterView; 3],
}

impl Default for SorterViewMap {
    fn default() -> Self {
        Self {
            inner: [
                SorterView::Simple(SimpleView::new(5)),
                SorterView::Simple(SimpleView::new(100)),
                SorterView::Simple(SimpleView::new(1000)),
            ],
        }
    }
}

impl Index<SorterViewId> for SorterViewMap {
    type Output = SorterView;

    fn index(&self, index: SorterViewId) -> &Self::Output {
        match index {
            SorterViewId::SimpleSmall => &self.inner[0],
            SorterViewId::Simple => &self.inner[1],
            SorterViewId::SimpleBig => &self.inner[2],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SorterViewId {
    SimpleSmall,
    Simple,
    SimpleBig,
}

impl fmt::Display for SorterViewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SimpleSmall => "Simple small",
            Self::Simple => "Simple",
            Self::SimpleBig => "Simple big",
        })
    }
}

impl SorterViewId {
    pub const fn all() -> [Self; 3] {
        [Self::SimpleSmall, Self::Simple, Self::SimpleBig]
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SorterView {
    Simple(SimpleView),
}

impl SorterView {
    pub fn slice(&self) -> SharedSlice {
        match self {
            Self::Simple(simple) => simple.slice(),
        }
    }

    pub fn view(&self, draw: &Draw, window_rect: Rect) {
        match self {
            Self::Simple(simple) => simple.view(draw, window_rect),
        }
    }
}
