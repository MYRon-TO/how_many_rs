#![allow(unused)]
mod counter;
mod visit;

use std::path::Path;

pub fn run(path: &Path, num: &mut u32) {
    // 创建计数器
    let mut counter = counter::Counter::new(8);
    visit::visit(path, num);
}
