use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::shuffle::Shuffle;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BogoSort;

impl Sort for BogoSort {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        while !slice.is_sorted().await {
            Shuffle.sort(slice).await;
        }
    }
}
