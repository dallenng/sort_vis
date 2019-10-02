use crate::array::Array;

pub fn merge_sort(array: Array) {
    let len = array.len();

    sub_merge_sort(&array, 0, len / 2);
    sub_merge_sort(&array, len / 2, len);

    merge(&array, 0, len / 2, len);
}

fn sub_merge_sort(array: &Array, left: usize, right: usize) {
    if right - left < 2 {
        return;
    }

    let mid = (left + right) / 2;

    sub_merge_sort(array, left, mid);
    sub_merge_sort(array, mid, right);

    merge(array, left, mid, right);
}

fn merge(array: &Array, left: usize, mid: usize, right: usize) {
    if array.get(mid - 1) < array.get(mid) {
        return;
    }
    let left_array = copy(array, left, mid);
    let left_len = left_array.len();
    let mut i = 0;
    let mut j = mid;
    let mut k = left;

    while i < left_len && j < right {
        if left_array[i] < array.get(j) {
            array.set(k, left_array[i]);
            i += 1;
        } else {
            array.set(k, array.get(j));
            j += 1;
        }
        k += 1;
    }

    while i < left_len {
        array.set(k, left_array[i]);
        i += 1;
        k += 1;
    }

    while j < right {
        array.set(k, array.get(j));
        j += 1;
        k += 1;
    }
}

fn copy(array: &Array, left: usize, right: usize) -> Vec<f32> {
    let mut vec = Vec::with_capacity(right - left);
    for i in left..right {
        vec.push(array.get(i));
    }
    vec
}
