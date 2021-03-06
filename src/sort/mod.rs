use crate::array::Array;
use crate::sort::bogo::bogo_sort;
use crate::sort::bubble::bubble_sort;
use crate::sort::heap::heap_sort;
use crate::sort::insertion::insertion_sort;
use crate::sort::merge::{in_place_merge_sort, merge_sort};
use crate::sort::selection::selection_sort;
use std::collections::HashMap;

pub mod bogo;
pub mod bubble;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod selection;

pub fn all_sort_functions() -> HashMap<&'static str, fn(Array)> {
    let mut map: HashMap<_, fn(Array)> = HashMap::new();
    map.insert("bubble", bubble_sort);
    map.insert("bogo", bogo_sort);
    map.insert("insertion", insertion_sort);
    map.insert("selection", selection_sort);
    map.insert("merge", merge_sort);
    map.insert("merge_in_place", in_place_merge_sort);
    map.insert("heap", heap_sort);
    map
}
