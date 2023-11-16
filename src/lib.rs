#![allow(unused)]
mod worker;

use self::worker::Worker;
use std::{
    path::Path,
    sync::{mpsc, Arc, Mutex},
};

pub enum Message {
    Done(u8),
    Quest(String),
    Answer(u32),
    ImWorking,
}
pub enum Approve {
    Work(String),
}

pub fn run(path: &Path, core_size: u8) {
    // visit::visit(path, num);

    // Worker::new(message_sender.clone(), Arc::clone(&listen)).work(path_s);
    let mut pool = ThreadPool::new(core_size);
    pool.run(path);
}

struct ThreadPool {
    workers: Vec<Worker>, // 线程池
    max: u8,              // 最大线程数（不需要？）
    // relaxing: Vec<u8>,
    relax: u8,
    message_receiver: mpsc::Receiver<Message>, // 消息发送：通知worker
    approve: Option<mpsc::Sender<Approve>>,    // 接收worker请求
    work_pool: Vec<String>,                    // 工作池
    count: u32,
    on_waiting: u32,
}

impl ThreadPool {
    pub fn new(max: u8) -> ThreadPool {
        let mut workers = Vec::with_capacity(max as usize);
        let mut relax: u8 = max;
        let mut work_pool = Vec::new();

        // message 收发通道，用来通知 worker
        let (message_sender, message_receiver): (mpsc::Sender<Message>, mpsc::Receiver<Message>) =
            mpsc::channel();

        // listen 收发通道，用来接收 worker 的通知
        let (approve, listen): (mpsc::Sender<Approve>, mpsc::Receiver<Approve>) = mpsc::channel();
        let listen = Arc::new(Mutex::new(listen));

        for i in 0..max {
            workers.push(Worker::new(i, message_sender.clone(), Arc::clone(&listen)));
        }

        ThreadPool {
            workers,
            max,
            relax,
            message_receiver,
            approve: Some(approve),
            work_pool,
            count: 0,
            on_waiting: 0,
        }
    }

    pub fn run(&mut self, path: &Path) {
        let mut path_s = String::from(path.to_str().unwrap());

        if let Some(sender) = &self.approve {
            sender.send(Approve::Work(path_s)).unwrap();
            self.on_waiting += 1;
            self.relax -= 1;
            for message in &self.message_receiver {
                match message {
                    Message::Quest(path) => {
                        // println!("New path: {}", path);
                        sender.send(Approve::Work(path)).unwrap();
                        self.on_waiting += 1;
                    }
                    Message::Answer(an) => {
                        self.count += an;
                        println!("now count: {}", self.count);
                    }
                    Message::Done(u8) => {
                        // self.relaxing.push(u8);
                        self.relax += 1;
                        // println!("Done");
                        // println!("relax: {}", self.relax);
                    }
                    Message::ImWorking => {
                        // println!("ImWorking");
                        println!("relax: {}", self.relax);
                        println!("on_waiting: {}", self.on_waiting);
                        if self.relax > 0 && self.on_waiting > 0 {
                            self.relax -= 1;
                            self.on_waiting -= 1;
                            // println!("relax: {}", self.relax);
                        } else {
                            println!("Error: relax < 0");
                        }
                    }
                }
                if self.relax == self.max && self.work_pool.len() == 0 && self.on_waiting == 0 {
                    break;
                }
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.approve.take());

        for worker in &mut self.workers {
            // println!("Shutting down worker {}", worker.get_id());
            if let Some(thread) = worker.work.take() {
                thread.join().unwrap();
            }
        }
        println!("Finally Answer: {}", self.count);
    }
}
