use crate::array::Array;

pub fn heap_sort(array: Array) {
    heapify(&array);

    let end = array.len() - 1;
    array.swap(end, 0);
    for i in (1..end).rev() {
        sift_down(&array, 0, i);
        array.swap(i, 0);
    }
}

fn heapify(array: &Array) {
    let end = array.len() - 1;

    for i in (0..=parent(end)).rev() {
        sift_down(array, i, end);
    }
}

fn sift_down(array: &Array, start: usize, end: usize) {
    let mut root = start;

    while left_child(root) <= end {
        let child = left_child(root);
        let mut swap = root;

        if array.get(swap) < array.get(child) {
            swap = child;
        }
        if child < end && array.get(swap) < array.get(child + 1) {
            swap = child + 1;
        }

        if swap == root {
            return;
        } else {
            array.swap(root, swap);
            root = swap;
        }
    }
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}

fn left_child(i: usize) -> usize {
    2 * i + 1
}
