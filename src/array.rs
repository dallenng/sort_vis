use crate::state::SharedState;
use std::thread;
use std::time::Duration;
use rand::thread_rng;
use rand::seq::SliceRandom;

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

    pub fn is_sorted(&self) -> bool {
        let state = self.0.get();
        let array = &state.array;
        for i in 1..array.len() {
            if array[i - 1] > array[i] {
                return false;
            }
        }
        true
    }

    pub fn shuffle(&self) {
        self.0.get().array.shuffle(&mut thread_rng())
    }

    pub fn wait(&self, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}