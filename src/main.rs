// #![allow(unused)]

use std::{
    collections::HashMap,
    path::Path,
};
use how_many_rs::visit;


fn main() {
    // 起始路径
    let path = Path::new("/Users/miaoyuanrong/");
    // 建立一个 HashMap 来存储文件类型和数量
    let mut files_types_num: HashMap<String, u32> = HashMap::new();

    // 访问目录
    visit::visit(path, &mut files_types_num);

    println!("{:?}", files_types_num);
}
