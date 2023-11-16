// #![allow(unused)]

use std::path::Path;
use how_many_rs as hr;

fn main() {
    // 起始路径
    // let path = Path::new("/home/Myron/Code/how_many_rs/i_am_test_datas/");
    let path = Path::new("/home/Myron/Code/how_many_rs/");
    // let path = Path::new("/home/Myron/Code/");
    // let path = Path::new("/home/Myron/");

    hr::run(path, 8);

}
