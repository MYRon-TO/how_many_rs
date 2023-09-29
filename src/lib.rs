pub mod visit {

    use std::{
        collections::HashMap,
        fs,
        // io,
        path::Path,
    };

    pub fn visit(path: &Path, files_types_num: &mut HashMap<String, u32>) {
        // 遍历目录
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();

            if path.is_dir() {
                // 递归访问子目录
                visit(&path, files_types_num);
                continue;
            } else if path.is_symlink() {
                // 跳过链接
                // println!("here is a link!");
                continue;
            } else {
                count_types(&path, files_types_num);
            }
        }
    }

    fn count_types(path: &Path, files_types_num: &mut HashMap<String, u32>) {
        match path.extension() {
            Some(file) => {
                // 有后缀名的文件
                let filetype = file.to_str().unwrap().to_string();
                let count = files_types_num.entry(filetype).or_insert(0);
                *count += 1;
            }
            None => {
                // 没有后缀名的文件
                let count = files_types_num.entry("empty".to_string()).or_insert(0);
                *count += 1;
            }
        }
    }
}
