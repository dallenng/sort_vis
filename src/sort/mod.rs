use crate::array::Array;

pub trait Sort {
    fn sort(array: Array);
}

pub mod bogosort;
pub mod bubble;
