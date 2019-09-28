use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct State {
    pub array: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct SharedState(Arc<Mutex<State>>);

impl State {
    pub fn new(size: u32) -> Self {
        let mut array: Vec<_> = (1..=size).collect();
        let mut array: Vec<_> = array
            .iter_mut()
            .map(|int| (*int as f32) / (size as f32))
            .collect();

        array.shuffle(&mut thread_rng());
        Self { array }
    }
}

impl SharedState {
    pub fn new(state: State) -> Self {
        Self(Arc::new(Mutex::new(state)))
    }

    pub fn get(&self) -> MutexGuard<State> {
        self.0.lock().unwrap()
    }
}
