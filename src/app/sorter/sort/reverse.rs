use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Reverse;

impl Sort for Reverse {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        let len = slice.len();
        let half_len = len / 2;

        for (i, j) in (0..half_len).zip((half_len..len).rev()) {
            slice.swap(i, j).await;
        }
    }
}
