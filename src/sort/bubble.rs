use crate::array::Array;
use crate::sort::Sort;

pub struct Bubble;

impl Sort for Bubble {
    fn sort(array: Array) {
        let mut len = array.len() - 1;
        let mut sorted = false;

        while !sorted && len > 1 {
            sorted = true;
            for i in 0..len {
                if array.get(i) > array.get(i + 1) {
                    array.swap(i, i + 1);
                    array.wait(30);
                    sorted = false;
                };
            }
            len -= 1;
        }
    }
}
