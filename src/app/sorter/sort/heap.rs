use std::cmp;

use crate::app::sorter::slice::Slice;
use crate::app::sorter::sort::Sort;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct HeapSort;

impl Sort for HeapSort {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord,
    {
        heapify(slice).await;

        slice.swap(0, slice.len() - 1).await;

        for i in (2..slice.len()).rev() {
            sift_down(&mut slice.slice(0..i), 0).await;
            slice.swap(0, i - 1).await;
        }
    }
}

async fn heapify<T>(slice: &mut impl Slice<T>)
where
    T: Ord,
{
    let last_parent = parent(slice.len() - 1);

    for i in (0..=last_parent).rev() {
        sift_down(slice, i).await;
    }
}

async fn sift_down<T>(slice: &mut impl Slice<T>, root: usize)
where
    T: Ord,
{
    let len = slice.len();
    let mut current = root;

    while left_child(current) < len {
        let swap = {
            let left_child_index = left_child(current);
            let left_child = slice.get(left_child_index).await;
            let right_child_index = left_child_index + 1;

            let (max_child_index, max_child) = if right_child_index < len {
                let right_child = slice.get(right_child_index).await;

                cmp::max_by(
                    (left_child_index, left_child),
                    (right_child_index, right_child),
                    |(_, a), (_, b)| a.cmp(b),
                )
            } else {
                (left_child_index, left_child)
            };

            cmp::max_by(
                (current, slice.get(current).await),
                (max_child_index, max_child),
                |(_, a), (_, b)| a.cmp(b),
            )
            .0
        };

        if current == swap {
            return;
        }

        slice.swap(current, swap).await;
        current = swap;
    }
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}

fn left_child(i: usize) -> usize {
    2 * i + 1
}
