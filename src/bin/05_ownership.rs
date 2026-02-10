/// Rust 所有权系统详解
/// 
/// 本文件演示了 Rust 中的所有权系统，包括：
/// 1. 所有权的基本规则
/// 2. 移动语义（Move Semantics）
/// 3. 借用（Borrowing）与引用
/// 4. 可变借用与不可变借用
/// 5. 借用规则
/// 6. 生命周期（Lifetimes）
/// 7. 切片类型（Slices）
/// 8. 所有权与函数
/// 9. 所有权与集合
/// 10. 实际应用示例

fn main() {
    println!("=== 1. 所有权的基本规则 ===");
    println!("Rust 所有权系统的三个核心规则：");
    println!("1. 每个值在 Rust 中都有一个唯一的所有者变量");
    println!("2. 当所有者变量离开作用域时，该值会被自动销毁");
    println!("3. 一个值一次只能被一个所有者持有，值可以从一个所有者移动到另一个所有者");

    println!("\n=== 2. 移动语义（Move Semantics）===");
    // 基本移动
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    
    let s2 = s1; // s1 的所有权移动到 s2
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // ❌ 编译错误：s1 已经无效
    
    // 移动与基本类型的区别
    let x = 5; // 基本类型（Copy trait）
    let y = x; // 复制，不是移动
    println!("x = {}, y = {}", x, y); // OK，x 仍然有效

    println!("\n=== 3. 克隆（Clone）===");
    // 克隆用于创建值的深拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝，创建新的字符串
    println!("s1 = {}, s2 = {}", s1, s2); // 两个都有效

    println!("\n=== 4. 借用（Borrowing）与引用 ===");
    // 不可变借用：使用 & 符号
    let s = String::from("hello");
    println!("Before borrow: s = {}", s);
    
    takes_ref(&s); // 传递不可变引用
    println!("After borrow: s = {}", s); // s 仍然有效

    println!("\n=== 5. 可变借用 ===");
    // 可变借用：使用 &mut 符号
    let mut s = String::from("hello");
    println!("Before mutable borrow: s = {}", s);
    
    change(&mut s); // 传递可变引用
    println!("After mutable borrow: s = {}", s); // s 已被修改

    println!("\n=== 6. 借用规则 ===");
    println!("Rust 的借用规则：");
    println!("1. 同一时间，一个值只能有一个可变引用或多个不可变引用");
    println!("2. 引用必须始终有效（不能引用已销毁的值）");
    
    // 示例：多个不可变引用
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("Multiple immutable references: r1 = {}, r2 = {}", r1, r2);
    
    // 示例：单个可变引用
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(", world");
        println!("Mutable reference: r1 = {}", r1);
    } // r1 离开作用域，借用结束
    
    println!("After mutable borrow: s = {}", s);
    
    // ❌ 编译错误：不能同时有可变和不可变引用
    /*
    let r1 = &s; // 不可变引用
    let r2 = &mut s; // 可变引用 - 编译错误
    println!("{}, {}", r1, r2);
    */

    println!("\n=== 7. 切片类型（Slices）===");
    // 切片是对集合一部分的引用，没有所有权
    let s = String::from("hello world");
    
    // 创建字符串切片
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("s = {}", s);
    println!("hello slice = {}", hello);
    println!("world slice = {}", world);
    
    // 切片作为函数参数
    let word = first_word(&s);
    println!("First word: {}", word);
    
    // 字符串字面量就是切片
    let literal = "hello world";
    println!("String literal: {}", literal);
    
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array: {:?}", a);
    println!("Array slice: {:?}", slice);

    println!("\n=== 8. 所有权与函数 ===");
    // 函数参数：移动
    let s1 = String::from("hello");
    println!("Before function call: s1 = {}", s1);
    takes_ownership(s1);
    // println!("After function call: s1 = {}", s1); // ❌ 编译错误：s1 已移动
    
    // 函数参数：借用
    let s2 = String::from("hello");
    println!("Before function call: s2 = {}", s2);
    borrows_ownership(&s2);
    println!("After function call: s2 = {}", s2); // OK
    
    // 函数返回值：移动
    let s3 = gives_ownership();
    println!("Received ownership: s3 = {}", s3);
    
    // 函数返回值：同时移动和借用
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("s5 = {}, length = {}", s5, len);

    println!("\n=== 9. 所有权与集合 ===");
    // Vec 的所有权
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", v);
    
    // 移动元素
    let first = v.remove(0);
    println!("Removed first element: {}, vector: {:?}", first, v);
    
    // 借用元素
    let third = &v[2];
    println!("Borrowed third element: {}, vector: {:?}", third, v);
    
    // 可变借用
    let last = &mut v[2];
    *last = 10;
    println!("Modified vector: {:?}", v);

    println!("\n=== 10. 实际应用示例 ===");
    // 示例：字符串处理
    let text = String::from("Rust is awesome!");
    println!("Original text: {}", text);
    
    let modified = process_text(&text);
    println!("Processed text: {}", modified);
    println!("Original text is still valid: {}", text);

    println!("\n=== 11. 所有权系统的优势 ===");
    println!("Rust 所有权系统的主要优势：");
    println!("1. 内存安全：无空指针、悬垂引用、数据竞争");
    println!("2. 零成本抽象：编译时检查，无运行时开销");
    println!("3. 无需垃圾回收：编译时自动管理内存");
    println!("4. 并发安全：借用规则防止数据竞争");
    println!("5. 代码清晰：明确的所有权转移和借用关系");
}

// ===============================================================================
// 辅助函数
// ===============================================================================

/// 接受不可变引用
fn takes_ref(s: &String) {
    println!("Got: {}", s);
}

/// 接受可变引用并修改值
fn change(s: &mut String) {
    s.push_str(", world!");
}

/// 查找字符串的第一个单词
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// 接受所有权（值被移动）
fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
}

/// 借用所有权（值不被移动）
fn borrows_ownership(s: &String) {
    println!("Borrowed: {}", s);
}

/// 返回所有权
fn gives_ownership() -> String {
    let s = String::from("hello");
    s // 返回 s，所有权被移动
}

/// 计算字符串长度并返回长度和所有权
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

/// 处理文本并返回新字符串
fn process_text(text: &str) -> String {
    let mut result = String::from("Processed: ");
    result.push_str(text);
    result
}

// ===============================================================================
// 与其他语言对比
// ===============================================================================

/*
对比其他语言：

- C/C++:
  - 手动内存管理（malloc/free, new/delete）
  - 容易出现内存泄漏、悬垂指针、双重释放
  - 无内置所有权系统

- Java:
  - 所有对象都是引用
  - 垃圾回收自动管理内存
  - 无显式所有权控制
  - 可能出现内存泄漏（长生命周期的对象持有短生命周期对象的引用）

- Python:
  - 引用计数 + 垃圾回收
  - 无显式所有权控制
  - 可能出现循环引用导致内存泄漏

- JavaScript:
  - 垃圾回收自动管理内存
  - 无显式所有权控制
  - 可能出现内存泄漏（闭包持有外部变量引用）

Rust 的特点：
- 编译时内存管理
- 无垃圾回收开销
- 明确的所有权转移
- 编译时借用检查
- 防止数据竞争
- 零成本抽象
*/
