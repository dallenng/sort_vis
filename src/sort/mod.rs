use crate::array::Array;

pub trait Sort {
    fn sort(array: Array);
}

pub mod bubble;
pub mod bogosort;
