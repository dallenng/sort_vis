use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BinaryInsertionSort;

impl Sort for BinaryInsertionSort {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        for i in 1..slice.len() {
            let j = {
                let (sorted, unsorted) = slice.split_at(i);
                let x = unsorted.get(0).await;
                sorted.binary_search(&x).await.unwrap_or_else(|i| i)
            };

            for j in (j..i).rev() {
                slice.swap(j, j + 1).await;
            }
        }
    }
}
