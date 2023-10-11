// #![allow(unused)]

use std::{
    thread,
    sync::mpsc,
};

/// # Worker
/// id: worker 的 id
/// thread: worker 的线程
pub struct Worker {
    id: u8,
    answer: mpsc::Sender<u32>,
    done: mpsc::Sender<u8>,
}

impl Worker {
    /// # new
    /// 创建一个 Worker
    /// id: worker 的 id
    pub fn new(
        id: u8,
        answer:mpsc::Sender<u32>,
        done: mpsc::Sender<u8>,
        ) -> Worker {

        Worker {
            id,
            answer,
            done,
        }
    }

    fn work(&self) {
        let answer = self.answer.clone();
        let thread = thread::spawn(move || {
            answer.send(1).unwrap();
        });

    }
}

/// BuildManagerError 是一个枚举类型，用来表示错误
pub enum BuildManagerError {
    WrongSize(String),
}

/// 为 BuildManagerError 实现 Debug trait
impl std::fmt::Debug for BuildManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BuildManagerError::WrongSize(s) => write!(f, "WrongSize: {}", s),
        }
    }
}
