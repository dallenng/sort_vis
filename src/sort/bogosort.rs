use crate::sort::Sort;
use crate::array::Array;

pub struct Bogosort;

impl Sort for Bogosort {
    fn sort(array: Array) {
        while !array.is_sorted() {
            array.shuffle();
            array.wait(1);
        }
    }
}
