#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tokio::sync::mpsc::{channel, Receiver, Sender};
// use std::sync::Arc;
// use parking_lot::{Mutex, MutexGuard, MappedMutexGuard};

#[derive(Debug)]
pub struct Stream<T> {
    rx: Receiver,
}

impl<T> Stream<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            closed: false,
        }
    }

    // add task to the stream
    pub fn write(&mut self, item: T) {
        if self.closed {
            return;
        }

        self.queue.push_back(item);
    }

    // close the stream
    pub fn close(&mut self) {
        self.queue = VecDeque::new();
        self.closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    // get next task from the stream
    pub fn next(&mut self) -> T {
        self.queue.pop_front().expect("Can't pop item from Stream")
    }
}