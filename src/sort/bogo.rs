use crate::array::Array;
use std::time::Duration;

pub fn bogo_sort(array: Array) {
    while !array.is_sorted() {
        array.shuffle();
        array.wait(Duration::from_millis(100));
    }
}
