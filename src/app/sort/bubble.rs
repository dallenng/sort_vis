use crate::app::sort::{Sort, Sortable};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Bubble;

impl Sort for Bubble {
    fn sort(&mut self, array: &mut impl Sortable) {
        let mut n = array.len();

        while n > 0 {
            let mut new_n = 0;

            for i in 1..n {
                if array[i - 1] > array[i] {
                    array.swap(i - 1, i);
                    new_n = i;
                }
            }

            n = new_n;
        }
    }
}
