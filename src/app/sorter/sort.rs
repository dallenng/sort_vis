use std::fmt;

use crate::app::sorter::slice::{SharedSlice, SharedSubSlice, Slice};
use crate::app::sorter::sort::binary_insertion::BinaryInsertionSort;
use crate::app::sorter::sort::bogo::BogoSort;
use crate::app::sorter::sort::bubble::BubbleSort;
use crate::app::sorter::sort::heap::HeapSort;
use crate::app::sorter::sort::insertion::InsertionSort;
use crate::app::sorter::sort::reverse::Reverse;
use crate::app::sorter::sort::selection::SelectionSort;
use crate::app::sorter::sort::shuffle::Shuffle;

mod binary_insertion;
mod bogo;
mod bubble;
mod heap;
mod insertion;
mod reverse;
mod selection;
mod shuffle;

trait Sort {
    async fn sort<T>(&mut self, slice: &mut impl Slice<T>)
    where
        T: Ord;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SortAlgorithm {
    Reverse,
    Shuffle,
    Bogo,
    Bubble,
    Insertion,
    BinaryInsertion,
    Selection,
    Heap,
}

impl fmt::Display for SortAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Reverse => "Reverse",
            Self::Shuffle => "Shuffle",
            Self::Bogo => "Bogo",
            Self::Bubble => "Bubble",
            Self::Insertion => "Insertion",
            Self::BinaryInsertion => "Binary insertion",
            Self::Selection => "Selection",
            Self::Heap => "Heap",
        })
    }
}

impl SortAlgorithm {
    pub const fn all() -> [Self; 8] {
        [
            Self::Reverse,
            Self::Shuffle,
            Self::Bogo,
            Self::Bubble,
            Self::Insertion,
            Self::BinaryInsertion,
            Self::Selection,
            Self::Heap,
        ]
    }

    pub async fn sort(self, mut slice: SharedSlice) {
        let mut slice = SharedSubSlice::from(&mut slice);

        match self {
            Self::Reverse => Reverse.sort(&mut slice).await,
            Self::Shuffle => Shuffle.sort(&mut slice).await,
            Self::Bogo => BogoSort.sort(&mut slice).await,
            Self::Bubble => BubbleSort.sort(&mut slice).await,
            Self::Insertion => InsertionSort.sort(&mut slice).await,
            Self::BinaryInsertion => BinaryInsertionSort.sort(&mut slice).await,
            Self::Selection => SelectionSort.sort(&mut slice).await,
            Self::Heap => HeapSort.sort(&mut slice).await,
        }
    }
}
