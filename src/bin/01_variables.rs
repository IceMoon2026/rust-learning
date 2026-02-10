/// Rust 中的变量默认是 **不可变的**（immutable）
/// 这与 Java/Python 不同（它们默认可变），但更安全
/// 
/// 本文件演示了 Rust 中变量的基本用法，包括：
/// 1. 不可变变量（默认）
/// 2. 可变变量（使用 mut 关键字）
/// 3. 变量遮蔽（Shadowing）
/// 4. 常量定义
/// 5. 类型注解

fn main() {
    println!("=== 1. 不可变变量（默认）===");
    // ✅ 不可变变量（推荐）：一旦赋值，不能修改
    let x = 5;
    println!("x = {}", x);
    // x = 6; // ❌ 编译错误：cannot assign twice to immutable variable
    // 不可变变量的好处：
    // - 防止意外修改
    // - 便于并发编程
    // - 提高代码可读性

    println!("\n=== 2. 可变变量 ===");
    // ✅ 显式声明可变变量：使用 mut 关键字
    let mut y = 10;
    println!("y = {}", y);
    y = 20; // 允许修改
    println!("y = {}", y);
    // 何时使用可变变量：
    // - 需要频繁修改的值
    // - 性能关键路径（减少内存分配）

    println!("\n=== 3. 变量遮蔽（Shadowing）===");
    // 🔁 变量遮蔽：用新值“遮蔽”旧变量
    // 特点：
    // - 创建新变量，而非修改旧变量
    // - 可以改变变量类型
    // - 作用域限于当前块
    let z = 5;
    println!("Initial z = {}", z);
    
    let z = z + 1; // 创建新变量 z，类型保持 i32
    println!("After first shadowing: z = {}", z);
    
    let z = z * 2; // 创建另一个新变量 z
    println!("After second shadowing: z = {}", z); // 输出 12
    
    // 变量遮蔽与可变变量的区别：
    // - 遮蔽：创建新变量，旧变量仍然存在但不可访问
    // - 可变：修改同一个变量的值

    println!("\n=== 4. 常量定义 ===");
    // ⚙️ 常量：使用 const 关键字，必须指定类型
    // 特点：
    // - 必须在编译时确定值
    // - 不可变
    // - 可以在任何作用域中定义
    // - 命名约定：使用全大写字母，下划线分隔
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
    // MAX_POINTS = 200_000; // ❌ 编译错误：cannot assign to const

    println!("\n=== 5. 类型注解 ===");
    // 📝 类型注解：显式指定变量类型
    // 虽然 Rust 有类型推断，但有时显式注解可以提高可读性
    let x: i32 = 10; // 显式指定为 32 位整数
    let y: f64 = 3.14; // 显式指定为 64 位浮点数
    let z: bool = true; // 显式指定为布尔值
    let s: String = String::from("Hello"); // 显式指定为 String 类型
    
    println!("x: {}, type: i32", x);
    println!("y: {}, type: f64", y);
    println!("z: {}, type: bool", z);
    println!("s: {}, type: String", s);

    println!("\n=== 6. 变量遮蔽中的类型改变 ===");
    // 变量遮蔽的一个强大特性：可以改变变量类型
    let message = "Hello";
    println!("message (string slice): {}", message);
    
    let message = message.to_string(); // 从 &str 变为 String
    println!("message (String): {}", message);

    // 🆚 对比其他语言：
    // 🆚 Java：没有遮蔽，只能重新赋值（需 mutable）
    // 🆚 Python：所有变量默认可变，无遮蔽概念
    // 🆚 C++：有类似的变量遮蔽，但语法和规则不同
}
