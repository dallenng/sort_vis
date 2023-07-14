use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct InsertionSort;

impl Sort for InsertionSort {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        for i in 1..slice.len() {
            let mut j = i;

            while j > 0 && *slice.get(j - 1).await > *slice.get(j).await {
                slice.swap(j, j - 1).await;

                j -= 1;
            }
        }
    }
}
