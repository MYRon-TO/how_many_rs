#![allow(unused)]
pub mod visit {

    use std::{fs, path::Path};

    mod threads {
        use std::thread;

        pub struct Manager {
            workers: Vec<thread::JoinHandle<()>>,
            count: u8,
        }
        pub enum BuildManagerError {
            WrongSize(String),
        }

        impl Manager {
            pub fn new(size: u8) -> Result<Manager, BuildManagerError> {
                if size <= 0 {
                    Err(BuildManagerError::WrongSize(String::from("size was too small!")))
                } else {
                    Ok(Manager{
                        workers: vec![],
                        count: size,
                    })
                }
            }
        }
    }

    fn visit(path: &Path, num: &mut u32) {
        // 遍历目录
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();

            if path.is_dir() {
                // 递归访问子目录
                visit(&path, num);
                continue;
            } else if path.is_symlink() {
                // 跳过链接
                // println!("here is a link!");
                continue;
            } else {
                count_types(&path, num);
            }
        }
    }

    fn count_types(path: &Path, num: &mut u32) {
        if let Some(file) = path.extension() {
            if file.to_str().unwrap() == "rs" {
                *num += 1;
            }
        }
    }

    pub fn run(path: &Path, num: &mut u32) {
        visit(path, num)
    }
}
