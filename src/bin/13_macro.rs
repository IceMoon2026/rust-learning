#![recursion_limit = "2048"]

/// Rust 宏详解
/// 
/// 本文件演示了 Rust 中的宏机制，包括：
/// 1. 声明式宏（Declarative Macros）
/// 2. 过程宏（Procedural Macros）
/// 3. 内置宏（Built-in Macros）
/// 4. 宏的最佳实践
/// 5. 与其他语言的对比

// ===============================================================================
// 1. 内置宏
// ===============================================================================

/*
Rust 提供了许多内置宏，用于简化常见的操作

常用的内置宏：
- vec!：创建动态数组
- println!：打印到标准输出
- dbg!：打印调试信息
- assert!：断言条件为真
- assert_eq!：断言两个值相等
- panic!：触发 panic
- format!：格式化字符串
- include!：包含文件内容
- concat!：拼接字符串
- env!：获取环境变量

注意：
- 内置宏不需要导入，可以直接使用
- 内置宏的名称以 ! 结尾
- 内置宏的参数可以是表达式
*/

fn demonstrate_builtin_macros() {
    println!("=== 1. 内置宏 ===");
    
    // 使用 vec! 创建动态数组
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);
    
    // 使用 println! 打印数组长度
    println!("Array length: {}", numbers.len());
    
    // 使用 dbg! 打印数组内容并调试
    let doubled = dbg!(numbers.iter().map(|x| x * 2).collect::<Vec<_>>());
    
    // 使用 assert! 验证结果
    assert!(!doubled.is_empty(), "Doubled array should not be empty");
    
    // 使用 assert_eq! 验证结果
    assert_eq!(doubled.len(), 5, "Doubled array should have 5 elements");
    
    // 使用 format! 格式化字符串
    let s = format!("The doubled array is {:?}", doubled);
    println!("{}", s);
    
    // 使用 concat! 拼接字符串
    let concatenated = concat!("Hello", ", ", "world!");
    println!("Concatenated string: {}", concatenated);
    
    // 使用 env! 获取环境变量
    let home_dir = env!("HOME");
    println!("Home directory: {}", home_dir);
    
    // 使用 include! 包含文件内容
    // let file_content = include!("file.txt");
    // println!("File content: {}", file_content);
}

// ===============================================================================
// 2. 声明式宏
// ===============================================================================

/*
声明式宏是 Rust 中最常用的宏类型，用于生成代码

声明式宏的语法：
- 使用 macro_rules! 定义宏
- 宏的参数使用 $ 开头，后跟参数名
- 宏的参数可以是表达式、类型、标识符等

使用场景：
- 当需要生成重复的代码时
- 当需要创建 DSL（领域特定语言）时
- 当需要简化复杂的操作时

注意：
- 声明式宏的语法比较复杂，需要多实践才能掌握
- 声明式宏的参数可以是任意 Rust 语法
- 声明式宏的展开是在编译期进行的
*/

// 示例：定义一个简单的宏
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

// 示例：定义一个带参数的宏
macro_rules! say_hello_to {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// 示例：定义一个带多个参数的宏
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// 示例：定义一个重复的宏
macro_rules! vec_from {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

fn demonstrate_declarative_macros() {
    println!("\n=== 2. 声明式宏 ===");
    
    // 使用简单的宏
    say_hello!();
    
    // 使用带参数的宏
    say_hello_to!("Alice");
    say_hello_to!("Bob");
    
    // 使用带多个参数的宏
    let sum = add!(1, 2);
    println!("Sum: {}", sum);
    
    // 使用重复的宏
    let v = vec_from!(1, 2, 3, 4, 5);
    println!("Vec from macro: {:?}", v);
}

// ===============================================================================
// 3. 过程宏
// ===============================================================================

/*
过程宏是 Rust 中的一种高级宏类型，用于生成代码

过程宏的类型：
- 派生宏（Derive Macros）：用于为结构体或枚举派生 trait
- 属性宏（Attribute Macros）：用于为结构体、枚举或函数添加属性
- 函数宏（Function-like Macros）：用于像函数一样调用的宏

使用场景：
- 当需要为结构体或枚举自动实现 trait 时
- 当需要为代码添加自定义属性时
- 当需要创建复杂的代码生成器时

注意：
- 过程宏需要使用 proc-macro 库
- 过程宏的实现比较复杂，需要多实践才能掌握
- 过程宏的展开是在编译期进行的
*/

// 示例：派生宏
// #[derive(Debug, Clone, Copy)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// 示例：属性宏
// #[derive(Debug)]
// #[my_attribute("value")]
// struct MyStruct {
//     // ...
// }

// 示例：函数宏
// let result = my_macro!(1, 2, 3);

fn demonstrate_procedural_macros() {
    println!("\n=== 3. 过程宏 ===");
    
    // 示例：派生宏
    // let p = Point { x: 1, y: 2 };
    // println!("Point: {:?}", p);
    // 
    // 示例：属性宏
    // let s = MyStruct { /* ... */ };
    // 
    // 示例：函数宏
    // let result = my_macro!(1, 2, 3);
    // println!("Result: {}", result);
}

// ===============================================================================
// 4. 宏的最佳实践
// ===============================================================================

/*
宏的最佳实践：
1. 尽量使用内置宏，避免重复造轮子
2. 当需要生成重复的代码时，使用声明式宏
3. 当需要为结构体或枚举自动实现 trait 时，使用派生宏
4. 当需要为代码添加自定义属性时，使用属性宏
5. 当需要创建复杂的代码生成器时，使用函数宏
6. 保持宏的简单性，避免过于复杂的宏
7. 为宏添加文档，说明宏的使用方法
8. 测试宏的正确性，确保宏的展开结果符合预期

注意：
- 宏的展开是在编译期进行的，不会影响程序的运行时性能
- 宏的展开结果可能会比较大，需要注意代码的可读性
- 宏的错误信息通常比较模糊，需要仔细调试
*/

// ===============================================================================
// 5. 与其他语言的对比
// ===============================================================================

/*
与其他语言的对比：
1. C++：使用预处理器宏，与 Rust 的宏类似
2. Python：使用装饰器和元编程，与 Rust 的宏类似
3. Java：使用注解和反射，与 Rust 的宏类似
4. Go：使用代码生成工具，与 Rust 的宏类似

Rust 宏的优势：
- 更安全的宏展开
- 更好的编译时检查
- 更灵活的宏语法
- 更好的代码可读性

Rust 宏的劣势：
- 更复杂的语法
- 更多的样板代码
- 学习曲线较陡
*/

// ===============================================================================
// 6. 宏的高级特性
// ===============================================================================

/*
宏的高级特性：
1. 宏的参数可以是任意 Rust 语法
2. 宏的参数可以是重复的
3. 宏的参数可以是递归的
4. 宏的参数可以是命名的
5. 宏的参数可以是可选的
6. 宏的参数可以是默认的
7. 宏的参数可以是类型参数
8. 宏的参数可以是生命周期参数
*/

// 示例：带命名参数的宏
macro_rules! create_struct {
    ($name:ident { $($field:ident: $ty:ty),* }) => {
        struct $name {
            $($field: $ty),*
        }
    };
}

// 示例：带可选参数的宏
macro_rules! print_message {
    ($message:expr) => {
        println!("{}", $message);
    };
    () => {
        println!("Default message");
    };
}

// 示例：带默认参数的宏
macro_rules! add_with_default {
    ($a:expr, $b:expr) => {
        $a + $b
    };
    ($a:expr) => {
        $a + 1
    };
}

fn demonstrate_advanced_macros() {
    println!("\n=== 4. 宏的高级特性 ===");
    
    // 示例：带命名参数的宏
    // create_struct!(Point { x: i32, y: i32 });
    // let p = Point { x: 1, y: 2 };
    // println!("Point: x={}, y={}", p.x, p.y);
    
    // 示例：带可选参数的宏
    print_message!("Hello, world!");
    print_message!();
    
    // 示例：带默认参数的宏
    let sum1 = add_with_default!(1, 2);
    let sum2 = add_with_default!(1);
    println!("Sum1: {}, Sum2: {}", sum1, sum2);
}

// ===============================================================================
// 主函数
// ===============================================================================

fn main() {
    // 演示内置宏
    demonstrate_builtin_macros();
    
    // 演示声明式宏
    demonstrate_declarative_macros();
    
    // 演示过程宏
    demonstrate_procedural_macros();
    
    // 演示宏的高级特性
    demonstrate_advanced_macros();
    
    println!("\n=== 5. 宏总结 ===");
    println!("1. 宏是 Rust 中的一种元编程机制，用于生成代码");
    println!("2. 声明式宏用于生成重复的代码");
    println!("3. 过程宏用于生成更复杂的代码");
    println!("4. 内置宏用于简化常见的操作");
    println!("5. 宏的展开是在编译期进行的，不会影响程序的运行时性能");
    println!("6. 宏的语法比较复杂，需要多实践才能掌握");
}

// ===============================================================================
// 宏的补充说明
// ===============================================================================

/*
宏的补充说明：
- 宏是 Rust 中的一种元编程机制，用于生成代码
- 宏的展开是在编译期进行的，不会影响程序的运行时性能
- 宏的展开结果可能会比较大，需要注意代码的可读性
- 宏的错误信息通常比较模糊，需要仔细调试
- 宏的参数可以是任意 Rust 语法
- 宏的参数可以是重复的
- 宏的参数可以是递归的
- 宏的参数可以是命名的
- 宏的参数可以是可选的
- 宏的参数可以是默认的
- 宏的参数可以是类型参数
- 宏的参数可以是生命周期参数

宏的常见错误：
- 宏的参数不匹配：宏的参数数量或类型不匹配
- 宏的展开错误：宏的展开结果不符合预期
- 宏的递归错误：宏的递归深度超过限制
- 宏的性能问题：宏的展开结果过大，导致编译时间过长

宏的调试：
- 使用 cargo expand 查看宏的展开结果
- 使用 rust-analyzer 查看宏的展开结果
- 使用 println! 打印调试信息
- 使用 dbg! 打印调试信息
*/

// 示例：宏的调试
// macro_rules! my_macro {
//     ($x:expr) => {
//         println!("x = {}", $x);
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo expand --example my_macro

// 示例：宏的性能
// macro_rules! generate_code {
//     ($n:expr) => {
//         $(println!("Hello, world!");)*
//     };
// }
// 
// fn main() {
//     generate_code!(10000);
// }

// 运行示例
// cargo build --release
// time ./target/release/my_program

// 示例：宏的可读性
// macro_rules! complex_macro {
//     ($($x:expr),*) => {
//         $(println!("x = {}", $x);)*
//     };
// }
// 
// fn main() {
//     complex_macro!(1, 2, 3, 4, 5);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
//     
//     #[test]
//     fn test_my_macro() {
//         assert_eq!(my_macro!(10), 20);
//     }
// }

// 运行示例
// cargo test

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可扩展性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可测试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可复用性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可移植性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的兼容性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的稳定性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可预测性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可理解性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可调试性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的性能
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可靠性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的安全性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
// }
// 
// fn main() {
//     let result = my_macro!(10);
//     println!("Result: {}", result);
// }

// 运行示例
// cargo run

// 示例：宏的可维护性
// macro_rules! my_macro {
//     ($x:expr) => {
//         $x * 2
//     };
