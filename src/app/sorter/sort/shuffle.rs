use nannou::rand;
use nannou::rand::Rng;

use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Shuffle;

impl Sort for Shuffle {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        let mut rng = rand::thread_rng();

        for i in (1..slice.len()).rev() {
            let j = rng.gen_range(0..=i);
            slice.swap(j, i).await;
        }
    }
}
