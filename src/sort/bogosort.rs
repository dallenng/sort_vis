use crate::array::Array;
use crate::sort::Sort;

pub struct Bogosort;

impl Sort for Bogosort {
    fn sort(array: Array) {
        while !array.is_sorted() {
            array.shuffle();
            array.wait(100);
        }
    }
}
