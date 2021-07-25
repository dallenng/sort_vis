use std::thread;
use std::time::Duration;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::state::SharedState;

pub struct Array(SharedState);

impl Array {
    pub fn new(state: SharedState) -> Self {
        Self(state)
    }

    pub fn get(&self, index: usize) -> f32 {
        self.0.get().access[index] = 1.0;
        let get = self.0.get().array[index];
        self.wait(Duration::from_millis(5));
        get
    }

    pub fn set(&self, index: usize, val: f32) {
        self.0.get().access[index] = 1.0;
        self.0.get().array[index] = val;
        self.wait(Duration::from_millis(15));
    }

    pub fn swap(&self, a: usize, b: usize) {
        self.0.get().access[a] = 1.0;
        self.0.get().access[b] = 1.0;
        self.0.get().array.swap(a, b);
        self.wait(Duration::from_millis(30));
    }

    pub fn shuffle(&self) {
        self.0.get().access.iter_mut().for_each(|val| *val = 1.0);
        self.0.get().array.shuffle(&mut thread_rng());
        self.wait(Duration::from_millis(100));
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

    pub fn wait(&self, duration: Duration) {
        let wait_factor = self.0.get().wait_factor;
        thread::sleep(duration.mul_f32(wait_factor));
    }
}
