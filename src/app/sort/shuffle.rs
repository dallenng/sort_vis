use nannou::rand::{thread_rng, Rng};

use crate::app::sort::{Sort, Sortable};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Shuffle;

impl Sort for Shuffle {
    fn sort(&mut self, array: &mut impl Sortable) {
        let mut rng = thread_rng();

        for i in (1..array.len()).rev() {
            array.swap(i, rng.gen_range(0..=i));
        }
    }
}
