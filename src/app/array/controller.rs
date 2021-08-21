use std::ops::{Index, IndexMut};
use std::sync::atomic::{AtomicU8, Ordering};

use flume::Sender;

use crate::app::message::Message;
use crate::app::sort::Sortable;

#[derive(Debug)]
pub struct ArrayController {
    array: Vec<u8>,
    sender: Sender<Message>,
    last_mut: (usize, AtomicU8),
}

impl ArrayController {
    pub fn new(array: Vec<u8>, sender: Sender<Message>) -> Self {
        let last_mut = (0, AtomicU8::new(array[0]));

        Self {
            array,
            sender,
            last_mut,
        }
    }

    fn send_set_if_last_mut_changed(&self) {
        let last_mut_value = self.array[self.last_mut.0];

        if last_mut_value != self.last_mut.1.swap(last_mut_value, Ordering::Relaxed) {
            self.sender
                .send(Message::set(self.last_mut.0, last_mut_value))
                .unwrap();
        }
    }
}

impl Index<usize> for ArrayController {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.send_set_if_last_mut_changed();

        self.sender.send(Message::get(index)).unwrap();
        &self.array[index]
    }
}

impl IndexMut<usize> for ArrayController {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.send_set_if_last_mut_changed();

        self.last_mut.0 = index;
        self.last_mut.1.store(self.array[index], Ordering::Relaxed);
        &mut self.array[index]
    }
}

impl Sortable for ArrayController {
    fn len(&self) -> usize {
        self.array.len()
    }
}

impl Drop for ArrayController {
    fn drop(&mut self) {
        self.send_set_if_last_mut_changed();
    }
}
