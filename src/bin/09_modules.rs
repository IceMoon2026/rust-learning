// 文件路径: src/bin/09_modules.rs

/// Rust 的模块系统用于组织代码
/// 本例展示如何定义和使用模块

// 定义模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist!");
        }
    }
}

// 使用 use 简化路径
use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}

// 📂 项目结构建议：
// src/
//   lib.rs       // 库 crate 入口
//   main.rs      // 二进制 crate 入口
//   front_of_house/
//     mod.rs     // 模块定义
//     hosting.rs // 子模块