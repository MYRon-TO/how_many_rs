pub mod worker;

use crate::visit;
use std::sync::mpsc;

pub struct Counter {
    num: u32,
    workers: Vec<worker::Worker>,
    communicate: (mpsc::Sender<u32>, mpsc::Receiver<u32>),
}

impl Counter {
    pub fn new(core_size: u8) -> Self {
        let mut workers = Vec::with_capacity(core_size.into());
        for i in 0..core_size {
            workers.push(worker::Worker::new(i));
        }
        Counter {
            num: 0,
            workers,
            communicate: mpsc::channel(),
        }
    }

    pub fn count(&self) {}
}
