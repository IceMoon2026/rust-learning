/// Rust 错误处理详解
/// 
/// 本文件演示了 Rust 中的错误处理机制，包括：
/// 1. panic! 宏
/// 2. Result 枚举
/// 3. ? 操作符
/// 4. 自定义错误类型
/// 5. 错误传播
/// 6. 错误处理的最佳实践
/// 7. 与其他语言的对比

use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;
use std::fmt;
use std::error::Error;

// ===============================================================================
// 1. panic! 宏
// ===============================================================================

/*
panic! 宏用于处理不可恢复的错误，会导致程序终止

使用场景：
- 程序遇到无法继续执行的错误
- 发生了逻辑错误
- 调试时需要立即终止程序

注意：
- panic! 会展开栈（默认），清理资源
- 可以通过设置 RUST_BACKTRACE=1 查看栈回溯
*/

fn demonstrate_panic() {
    // 示例：panic! 宏
    // panic!("crash and burn"); // 程序终止
    
    // 示例：数组越界会触发 panic!
    // let v = vec![1, 2, 3];
    // v[99]; // 数组越界，触发 panic!
}

// ===============================================================================
// 2. Result 枚举
// ===============================================================================

/*
Result 枚举用于处理可恢复的错误，定义如下：
enum Result<T, E> {
    Ok(T),
    Err(E),
}

使用场景：
- 程序遇到可以恢复的错误
- 需要处理错误并继续执行
- 需要返回错误信息

注意：
- Result 是 Rust 中处理错误的主要方式
- 必须显式处理 Result，否则会编译警告
*/

fn demonstrate_result() {
    println!("=== 1. Result 枚举 ===");
    
    // 示例：打开文件
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };
    
    println!("File opened or created successfully!");
}

// ===============================================================================
// 3. ? 操作符
// ===============================================================================

/*
? 操作符用于简化错误传播，相当于 match 表达式的简化形式

使用场景：
- 在返回 Result 的函数中传播错误
- 简化错误处理代码

注意：
- ? 操作符只能用于返回 Result 或 Option 的函数
- ? 操作符会将错误转换为函数返回的错误类型
*/

fn demonstrate_question_mark() {
    println!("\n=== 2. ? 操作符 ===");
    
    // 示例：使用 ? 操作符简化错误处理
    let f2 = open_file("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap()
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });
    
    println!("File opened or created successfully using ?!");
    
    // 示例：在返回 Result 的函数中使用 ?
    match read_username_from_file() {
        Ok(username) => println!("Username read from file: {}", username),
        Err(error) => println!("Error reading username: {:?}", error),
    }
}

// 使用 ? 操作符的函数
fn open_file(filename: &str) -> Result<File, std::io::Error> {
    let f = File::open(filename)?;
    Ok(f)
}

// 使用 ? 操作符的函数
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut f: File = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ===============================================================================
// 4. 自定义错误类型
// ===============================================================================

/*
可以自定义错误类型，通常使用 enum 或 struct

使用场景：
- 需要表示多种不同的错误
- 需要为错误添加额外的信息
- 需要实现自定义的错误处理逻辑

注意：
- 自定义错误类型需要实现 std::error::Error trait
- 通常需要实现 Debug 和 Display trait
*/

// 自定义错误类型
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomError(String),
}

// 实现 Display trait
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::ParseError(e) => write!(f, "Parse error: {}", e),
            MyError::CustomError(s) => write!(f, "Custom error: {}", s),
        }
    }
}

// 实现 Error trait
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::IoError(e) => Some(e),
            MyError::ParseError(e) => Some(e),
            MyError::CustomError(_) => None,
        }
    }
}

// 转换：从 std::io::Error 转换为 MyError
impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError(error)
    }
}

// 转换：从 std::num::ParseIntError 转换为 MyError
impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

// 使用自定义错误类型的函数
fn read_number_from_file(filename: &str) -> Result<i32, MyError> {
    let mut f = File::open(filename)?; // 自动转换为 MyError
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 自动转换为 MyError
    let number = s.trim().parse()?; // 自动转换为 MyError
    Ok(number)
}

fn demonstrate_custom_error() {
    println!("\n=== 3. 自定义错误类型 ===");
    
    // 写入测试数据
    let mut f = File::create("number.txt").unwrap();
    f.write_all(b"42").unwrap();
    
    // 读取数字
    match read_number_from_file("number.txt") {
        Ok(number) => println!("Number read from file: {}", number),
        Err(error) => println!("Error reading number: {}", error),
    }
    
    // 测试自定义错误
    let result: Result<i32, MyError> = Err(MyError::CustomError(String::from("Something went wrong")));
    match result {
        Ok(_) => (),
        Err(error) => println!("Custom error: {}", error),
    }
}

// ===============================================================================
// 5. 错误传播
// ===============================================================================

/*
错误传播是指将错误从一个函数传递到另一个函数

使用场景：
- 上层函数需要处理下层函数的错误
- 需要在多个层次处理错误
- 需要将错误信息传递给用户

注意：
- 使用 ? 操作符可以简化错误传播
- 可以使用 map_err 转换错误类型
*/

fn demonstrate_error_propagation() {
    println!("\n=== 4. 错误传播 ===");
    
    // 示例：错误传播
    let result = process_file("hello.txt");
    match result {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error processing file: {}", error),
    }
}

// 处理文件的函数
fn process_file(filename: &str) -> Result<String, MyError> {
    let content = read_file(filename)?; // 错误传播
    let processed = process_content(&content); // 处理内容
    Ok(processed)
}

// 读取文件的函数
fn read_file(filename: &str) -> Result<String, MyError> {
    let mut f = File::open(filename)?; // 错误传播
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 错误传播
    Ok(s)
}

// 处理内容的函数
fn process_content(content: &str) -> String {
    content.to_uppercase()
}

// ===============================================================================
// 6. 错误处理的最佳实践
// ===============================================================================

/*
错误处理的最佳实践：
1. 优先使用 Result 处理可恢复的错误
2. 仅在无法恢复时使用 panic!
3. 使用 ? 操作符简化错误传播
4. 自定义错误类型以提供更丰富的错误信息
5. 实现 Display 和 Error trait 以提供友好的错误信息
6. 使用 map_err 转换错误类型
7. 避免过度使用 unwrap() 和 expect()
8. 为错误添加上下文信息
*/

// ===============================================================================
// 7. 与其他语言的对比
// ===============================================================================

/*
与其他语言的对比：
1. Java：使用 try/catch 处理错误，Rust 使用 Result + match/?
2. Python：使用 try/except 处理错误，Rust 使用 Result + match/?
3. C++：使用异常处理错误，Rust 使用 Result + match/?
4. Go：使用返回值处理错误，与 Rust 的 Result 类似

Rust 错误处理的优势：
- 更清晰的错误处理
- 更安全的错误处理
- 更好的性能
- 更好的编译时检查

Rust 错误处理的劣势：
- 更繁琐的错误处理
- 更多的样板代码
*/

// ===============================================================================
// 8. 错误处理的高级特性
// ===============================================================================

/*
错误处理的高级特性：
1. 错误链：使用 source() 方法获取原始错误
2. 错误上下文：使用 context() 方法添加上下文信息
3. 错误转换：使用 from() 方法转换错误类型
4. 错误聚合：使用 collect() 方法聚合错误
5. 异步错误处理：使用 ? 操作符处理异步错误
*/

// 示例：错误链
fn demonstrate_error_chain() {
    println!("\n=== 5. 错误链 ===");
    
    let result = read_number_from_file("nonexistent.txt");
    match result {
        Ok(_) => (),
        Err(error) => {
            println!("Error: {}", error);
            if let Some(source) = error.source() {
                println!("Source error: {}", source);
            }
        },
    }
}

// ===============================================================================
// 主函数
// ===============================================================================

fn main() {
    // 演示 panic! 宏
    // demonstrate_panic();
    
    // 演示 Result 枚举
    demonstrate_result();
    
    // 演示 ? 操作符
    demonstrate_question_mark();
    
    // 演示自定义错误类型
    demonstrate_custom_error();
    
    // 演示错误传播
    demonstrate_error_propagation();
    
    // 演示错误链
    demonstrate_error_chain();
    
    println!("\n=== 6. 错误处理总结 ===");
    println!("1. panic! 用于不可恢复的错误");
    println!("2. Result 用于可恢复的错误");
    println!("3. ? 操作符简化错误传播");
    println!("4. 可以自定义错误类型");
    println!("5. 错误处理是 Rust 安全的重要组成部分");
    println!("6. 必须显式处理错误，否则会编译警告");
}

// ===============================================================================
// 错误处理的补充说明
// ===============================================================================

/*
错误处理的补充说明：
- Rust 没有异常，所有错误都通过 Result 或 panic! 处理
- Rust 强制处理错误，避免了未处理的异常
- Rust 的错误处理是类型安全的
- Rust 的错误处理是零成本的（在编译时处理）

常用的错误处理方法：
- unwrap()：如果 Result 是 Ok，则返回值；如果是 Err，则 panic!
- expect()：与 unwrap() 类似，但可以自定义错误信息
- unwrap_or()：如果 Result 是 Ok，则返回值；如果是 Err，则返回默认值
- unwrap_or_else()：如果 Result 是 Ok，则返回值；如果是 Err，则调用闭包
- map()：如果 Result 是 Ok，则对值应用函数；如果是 Err，则返回 Err
- and_then()：如果 Result 是 Ok，则对值应用函数并返回 Result；如果是 Err，则返回 Err
*/

// 示例：常用的错误处理方法
fn demonstrate_error_methods() {
    // 为每个方法创建独立的 Result
    
    // unwrap()
    let result: Result<i32, MyError> = Ok(42);
    // let value = result.unwrap(); // 42
    
    let result2: Result<i32, MyError> = Err(MyError::CustomError(String::from("Error")));
    // let value2 = result2.unwrap(); // panic!
    
    // expect()
    let result3: Result<i32, MyError> = Ok(42);
    // let value = result3.expect("Expected a number"); // 42
    
    let result4: Result<i32, MyError> = Err(MyError::CustomError(String::from("Error")));
    // let value2 = result4.expect("Expected a number"); // panic! with message
    
    // unwrap_or()
    let result5: Result<i32, MyError> = Ok(42);
    let _value = result5.unwrap_or(0); // 42
    
    let result6: Result<i32, MyError> = Err(MyError::CustomError(String::from("Error")));
    let _value2 = result6.unwrap_or(0); // 0
    
    // unwrap_or_else()
    let result7: Result<i32, MyError> = Ok(42);
    let _value3 = result7.unwrap_or_else(|_| 0); // 42
    
    let result8: Result<i32, MyError> = Err(MyError::CustomError(String::from("Error")));
    let _value4 = result8.unwrap_or_else(|_| 0); // 0
    
    // map()
    let result9: Result<i32, MyError> = Ok(42);
    let _mapped = result9.map(|x| x * 2); // Ok(84)
    
    let result10: Result<i32, MyError> = Err(MyError::CustomError(String::from("Error")));
    let _mapped2 = result10.map(|x| x * 2); // Err(MyError)
    
    // and_then()
    let result11: Result<i32, MyError> = Ok(42);
    let _and_then = result11.and_then(|x| Ok(x * 2)); // Ok(84)
    
    let result12: Result<i32, MyError> = Err(MyError::CustomError(String::from("Error")));
    let _and_then2 = result12.and_then(|x| Ok(x * 2)); // Err(MyError)
}

// 运行示例
// demonstrate_error_methods();
