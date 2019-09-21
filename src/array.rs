use crate::state::SharedState;
use std::thread;
use std::time::Duration;

pub struct Array(SharedState);

impl Array {
    pub fn new(state: SharedState) -> Self {
        Self(state)
    }

    pub fn get(&self, index: usize) -> u32 {
        self.0.get().array[index]
    }

    pub fn set(&self, index: usize, val: u32) {
        self.0.get().array[index] = val;
    }

    pub fn swap(&self, a: usize, b: usize) {
        self.0.get().array.swap(a, b);
    }

    pub fn len(&self) -> usize {
        self.0.get().array.len()
    }

    pub fn wait(&self, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}
