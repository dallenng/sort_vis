use crate::array::Array;

pub fn bogo_sort(array: Array) {
    while !array.is_sorted() {
        array.shuffle();
    }
}
