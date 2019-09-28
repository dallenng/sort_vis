use crate::state::SharedState;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::thread;
use std::time::Duration;

pub struct Array(SharedState);

impl Array {
    pub fn new(state: SharedState) -> Self {
        Self(state)
    }

    pub fn get(&self, index: usize) -> f32 {
        self.0.get().array[index]
    }

    pub fn set(&self, index: usize, val: f32) {
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

    pub fn wait(&self, us: u64) {
        thread::sleep(Duration::from_micros(us));
    }
}
