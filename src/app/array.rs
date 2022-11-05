use std::time::Duration;

use flume::{Receiver, TryRecvError};
use threadpool::ThreadPool;

use crate::app::array::controller::ArrayController;
use crate::app::message::Message;
use crate::app::sort::Sort;

pub mod controller;
pub mod view;

#[derive(Debug, Clone)]
pub struct Array {
    inner: Vec<u8>,
    access: Vec<f32>,
    len: f32,
    receiver: Option<Receiver<Message>>,
    sort_threads: ThreadPool,
}

impl Array {
    pub fn new(len: u8) -> Self {
        Self {
            inner: (1..=len).collect(),
            access: vec![0.0; len as usize],
            len: f32::from(len),
            receiver: None,
            sort_threads: threadpool::Builder::new()
                .num_threads(2)
                .thread_name("Sort thread".to_owned())
                .build(),
        }
    }

    pub fn update(&mut self, steps: u8, delta: Duration) {
        let delta = delta.as_secs_f32()
            * (self.access.iter().sum::<f32>() * (f32::from(steps) + 10.0) / self.len);
        for access in self.access.iter_mut().filter(|x| **x >= delta) {
            *access -= delta;
        }

        for _ in 0..steps {
            if let Some(receiver) = &self.receiver {
                let msg = match receiver.try_recv() {
                    Ok(msg) => msg,
                    Err(TryRecvError::Empty) => continue,
                    Err(TryRecvError::Disconnected) => break,
                };

                match msg {
                    Message::Get { index } => self.access[index] = 1.0,
                    Message::Set { index, value } => {
                        self.access[index] = 1.0;
                        self.inner[index] = value;
                    }
                }
            }
        }
    }

    pub fn sort(&mut self, mut algorithm: impl Sort + Send + 'static) {
        let (sender, receiver) = flume::unbounded();
        self.receiver.replace(receiver);

        let mut controller = ArrayController::new(self.inner.clone(), sender);
        self.sort_threads.execute(move || algorithm.sort(&mut controller));
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        self.receiver.take();
        self.sort_threads.join();
    }
}
