use std::ops::{Index, IndexMut, Range};

pub mod bubble;
pub mod shuffle;

// #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
// pub enum SortAlgorithm {
//     Shuffle(shuffle::Shuffle),
//     Bubble(bubble::Bubble),
// }
//
// impl Sort for SortAlgorithm {
//     fn sort(&mut self, array: &mut impl Sortable) {
//         match self {
//             SortAlgorithm::Bubble(bubble) => bubble.sort(array),
//             SortAlgorithm::Shuffle(shuffle) => shuffle.sort(array),
//         }
//     }
// }

pub trait Sort {
    fn sort(&mut self, array: &mut impl Sortable);
}

pub trait Sortable: Index<usize, Output = u8> + IndexMut<usize, Output = u8> {
    fn len(&self) -> usize;

    fn swap(&mut self, a: usize, b: usize) {
        let tmp = self[a];
        self[a] = self[b];
        self[b] = tmp;
    }

    fn is_sorted(&self) -> bool {
        self.is_sorted_range(0..self.len())
    }

    fn is_sorted_range(&self, range: Range<usize>) -> bool {
        range.clone().zip(range.skip(1)).all(|(a, b)| self[a] <= self[b])
    }

    fn binary_search(&self, x: u8) -> Result<usize, usize> {
        self.binary_search_range(x, 0..self.len())
    }

    fn binary_search_range(&self, x: u8, range: Range<usize>) -> Result<usize, usize> {
        dbg!(x);
        dbg!(range);
        todo!()
    }
}
