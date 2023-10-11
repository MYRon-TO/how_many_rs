pub mod worker;

use crate::visit;
use std::sync::mpsc;

/// # Counter
/// num: workers 的数量
/// workers: 存储 worker 的 vector
/// answer_giver: 用来接收 worker 的答案
/// done_work: 用来接收 worker 的完成信号
/// count: 计数器
pub struct Counter {
    num: u32,
    workers: Vec<worker::Worker>,
    answer_giver: mpsc::Receiver<u32>,
    done_work: mpsc::Receiver<u8>,
    count: u32,
}

impl Counter {
    /// # new
    /// 创建一个 Counter
    /// core_size: worker 的数量
    pub fn new(core_size: u8) -> Self {
        let mut workers = Vec::with_capacity(core_size.into());
        let (answer, answer_giver) = mpsc::channel();
        let (done, done_work) = mpsc::channel();
        for i in 0..core_size {
            workers.push(
                worker::Worker::new(
                    i,
                    answer.clone(),
                    done.clone(),
                    )
                );
        }
        Counter {
            num: 0,
            workers,
            count: 0,
            answer_giver,
            done_work,
        }
    }

    pub fn count(&mut self) {
        for answer in &self.answer_giver {
            self.count += answer;
        }
        println!("{}", self.count);
    }
}
