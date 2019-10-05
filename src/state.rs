use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct State {
    pub array: Vec<f32>,
    pub access: Vec<f32>,
    pub wait_factor: f32,
}

#[derive(Debug, Clone)]
pub struct SharedState(Arc<Mutex<State>>);

impl State {
    pub fn new(size: usize) -> Self {
        let mut array: Vec<_> = (1..=size).collect();
        let mut array: Vec<_> = array
            .iter_mut()
            .map(|int| (*int as f32) / (size as f32))
            .collect();

        array.shuffle(&mut thread_rng());

        let access = (0..size).map(|_| 0f32).collect();
        Self {
            array,
            access,
            wait_factor: 1.0,
        }
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
