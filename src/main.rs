// #![allow(unused)]

use std::path::Path;
use how_many_rs as hr;

fn main() {
    // 起始路径
    let path = Path::new("/home/Myron/Code/how_many_rs/i_am_test_datas/");
    // 建立一个 HashMap 来存储文件类型和数量
    // let mut files_types_num: HashMap<String, u32> = HashMap::new();
    let mut num: u32 = 0;

    // 访问目录
    hr::run(path, &mut num);

    println!("{}", num);
}
