// #![allow(unused)]

use std::{
    thread,
    sync::mpsc,
};

pub struct Worker {
    id: u8,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: u8) -> Worker {
        let thread = thread::spawn(move || {
            println!("I'm a worker!");
        });

        Worker {
            id,
            thread,
        }
    }
}

/// BuildManagerError 是一个枚举类型，用来表示错误
pub enum BuildManagerError {
    WrongSize(String),
}

impl std::fmt::Debug for BuildManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BuildManagerError::WrongSize(s) => write!(f, "WrongSize: {}", s),
        }
    }
}
