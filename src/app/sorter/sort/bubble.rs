use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BubbleSort;

impl Sort for BubbleSort {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        let mut n = slice.len();

        while n > 0 {
            let mut new_n = 0;

            for i in 1..n {
                if *slice.get(i - 1).await > *slice.get(i).await {
                    slice.swap(i - 1, i).await;
                    new_n = i;
                }
            }

            n = new_n;
        }
    }
}
