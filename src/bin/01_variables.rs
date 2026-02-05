/// Rust 中的变量默认是 **不可变的**（immutable）
/// 这与 Java/Python 不同（它们默认可变），但更安全
fn main() {
    // ✅ 不可变变量（推荐）
    let x = 5;
    println!("x = {}", x);
    // x = 6; // ❌ 编译错误：cannot assign twice to immutable variable

    // ✅ 显式声明可变变量
    let mut y = 10;
    println!("y = {}", y);
    y = 20; // 允许修改
    println!("y = {}", y);

    // 🔁 变量遮蔽（Shadowing）—— 用新值“遮蔽”旧变量
    let z = 5;
    let z = z + 1; // 创建新变量 z，类型可不同！
    let z = z * 2;
    println!("z = {}", z); // 输出 12

    // 🆚 对比 Java：Java 没有遮蔽，只能重新赋值（需 mutable）
    // 🆚 对比 Python：所有变量默认可变，无遮蔽概念
}