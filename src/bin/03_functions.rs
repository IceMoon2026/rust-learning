/// 函数定义：fn 关键字，参数需显式类型，返回值用 -> 声明
fn main() {
    another_function(5, 'h');
    
    plus_one(5);
    // println!("x = {}", x);

    // 表达式 vs 语句
    let y = {
        let a = 3;
        a + 1 // 注意：没有分号！这是表达式
    };
    println!("y = {}", y); // 4
}

/// 参数必须标注类型！
fn another_function(x: i32, unit_label: char) {
    println!("x = {}, label = {}", x, unit_label);
}

/// 返回值：最后一行不加分号即为返回值
fn plus_one(_x: i32) -> () {
    // x + 1 // 不能写成 x + 1;
}

// 🆚 对比 Java：
// - Java 函数叫 method（在 class 内），Rust 函数是独立的
// - Rust 无 void，用 () 表示无返回值