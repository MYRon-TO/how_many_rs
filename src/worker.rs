use std::{
    fs,
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
    // time,
};

use crate::{Approve, Message};

/// # Worker
/// id: worker 的 id
/// thread: worker 的线程
pub struct Worker {
    id: u8,
    pub work: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// # new
    /// 创建一个 Worker
    /// id: worker 的 id
    pub fn get_id (&self) -> u8 {
        self.id
    }

    pub fn new(
        id: u8,
        sender: mpsc::Sender<Message>,
        listen: Arc<Mutex<mpsc::Receiver<Approve>>>,
    ) -> Worker {
        // 创建一个线程
        let work = thread::spawn(move || loop {
            // println!("Worker {} is ready", id);

            // 接收消息
            let message = listen.lock().unwrap().recv();
            match message {
                Ok(Approve::Work(path)) => {
                    sender.send(Message::ImWorking).unwrap();
                    visit(path, sender.clone());
                    // println!("Worker {} is Done!", id);
                    sender.send(Message::Done(id)).unwrap();
                }
                Err(_) => {
                    // println!("Worker {} is shutting down", id);
                    break;
                }
            }
        });

        Worker { id, work:Some(work) }
    }
}

fn visit(
    path_s: String,
    sender: mpsc::Sender<Message>,
) {
    // 遍历目录
    let path = Path::new(path_s.as_str());
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            // println!("New dir");
            let path = path.to_str().unwrap().to_string();
            sender.send(Message::Quest(path)).unwrap();
        } else if path.is_symlink() {
            continue;
        } else {
            if count_types(&path) {
                // println!("New one");
                sender.send(Message::Answer(1)).unwrap();
            }
        }
    }
}

fn count_types(path: &Path) -> bool {
    if let Some(file) = path.extension() {
        if file.to_str().unwrap() == "rs" {
            return true;
        }
    }
    return false;
}
