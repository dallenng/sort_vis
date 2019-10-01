use crate::array::Array;

pub fn insertion_sort(array: Array) {
    for i in 1..array.len() {
        let x = array.get(i);

        let mut j = i - 1;
        while j < i && array.get(j) > x {
            array.set(j + 1, array.get(j));
            j -= 1;
        }

        array.set(j + 1, x);
    }
}
