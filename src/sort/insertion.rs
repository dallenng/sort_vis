use crate::array::Array;
use std::time::Duration;

pub fn insertion_sort(array: Array) {
    let len = array.len() as i32;

    let mut i = 1;
    while i < len {
        let x = array.get(i as usize);
        let mut j = i - 1;
        while j >= 0 && array.get(j as usize) > x {
            array.set((j + 1) as usize, array.get(j as usize));
            array.wait(Duration::from_millis(30));
            j -= 1;
        }
        array.set((j + 1) as usize, x);
        array.wait(Duration::from_millis(30));
        i += 1;
    }
}
