use crate::array::Array;

pub fn bubble_sort(array: Array) {
    let mut len = array.len() - 1;
    let mut sorted = false;

    while !sorted && len > 0 {
        sorted = true;
        for i in 0..len {
            if array.get(i) > array.get(i + 1) {
                array.swap(i, i + 1);
                sorted = false;
            };
        }
        len -= 1;
    }
}
