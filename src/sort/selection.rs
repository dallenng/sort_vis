use crate::array::Array;

pub fn selection_sort(array: Array) {
    let len = array.len();

    for i in 0..(len - 1) {
        let mut min = i;

        for j in (i + 1)..len {
            if array.get(j) < array.get(min) {
                min = j;
            }
        }

        if min != i {
            array.swap(i, min);
        }
    }
}
