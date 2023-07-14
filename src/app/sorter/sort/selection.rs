use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct SelectionSort;

impl Sort for SelectionSort {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        let len = slice.len();

        for i in 0..len - 1 {
            let mut min_index = i;

            {
                let mut min = slice.get(min_index).await;

                for j in i + 1..len {
                    let item = slice.get(j).await;

                    if *item < *min {
                        min_index = j;
                        min = item;
                    }
                }
            }

            if min_index != i {
                slice.swap(i, min_index).await;
            }
        }
    }
}
