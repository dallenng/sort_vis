use crate::array::Array;

pub fn bubble_sort(array: Array) {
    let mut sorted = false;

    for i in (1..array.len()).rev() {
        if sorted {
            break;
        }
        sorted = true;
        for j in 0..i {
            if array.get(j) > array.get(j + 1) {
                array.swap(j, j + 1);
                sorted = false;
            }
        }
    }
}
