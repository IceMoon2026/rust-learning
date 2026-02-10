/// Rust 控制流详解
/// 
/// 本文件演示了 Rust 中的各种控制流结构：
/// 1. if 表达式
/// 2. if let 表达式（模式匹配的简化形式）
/// 3. loop 循环
/// 4. while 循环
/// 5. for 循环
/// 6. match 表达式
/// 7. 控制流标签
/// 8. 控制流与表达式

fn main() {
    println!("=== 1. if 表达式 ===");
    // 基本 if-else
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 多个条件分支
    let score = 85;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: D");
    }

    // if 可作为表达式（必须有 else，且分支类型一致）
    let is_true = true;
    let number = if is_true { 5 } else { 6 };
    println!("number = {}", number);

    // if 表达式中的复杂逻辑
    let temperature = 25;
    let weather = if temperature > 30 {
        "hot"
    } else if temperature > 20 {
        "warm"
    } else if temperature > 10 {
        "cool"
    } else {
        "cold"
    };
    println!("Weather: {}", weather);

    println!("\n=== 2. if let 表达式 ===");
    // if let 是 match 的简化形式，用于只关心一种模式的情况
    let some_value = Some(5);
    
    // 使用 match
    match some_value {
        Some(x) => println!("Got value: {}", x),
        _ => println!("Got nothing"),
    }
    
    // 使用 if let 简化
    if let Some(x) = some_value {
        println!("if let: Got value: {}", x);
    } else {
        println!("if let: Got nothing");
    }
    
    // if let 与 else if let
    let another_value: Option<i32> = None;
    if let Some(x) = another_value {
        println!("Got Some({})", x);
    } else if let None = another_value {
        println!("Got None");
    }

    println!("\n=== 3. loop 循环 ===");
    // 基本 loop 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("Loop iteration: {}", counter);
        
        if counter == 5 {
            break counter * 2; // loop 可返回值
        }
    };
    println!("Loop result = {}", result); // 10

    // 使用 continue 跳过当前迭代
    let mut count = 0;
    loop {
        count += 1;
        
        if count % 2 == 0 {
            println!("Skipping even number: {}", count);
            continue; // 跳过当前迭代的剩余部分
        }
        
        println!("Processing odd number: {}", count);
        
        if count >= 10 {
            break;
        }
    }

    println!("\n=== 4. while 循环 ===");
    // 基本 while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // while let 循环（结合模式匹配）
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    println!("\n=== 5. for 循环 ===");
    // 遍历数组
    let a = [10, 20, 30, 40, 50];
    println!("Array elements:");
    for element in a.iter() {
        println!("value: {}", element);
    }

    // 遍历范围（左闭右开）
    println!("\nRange 1..5:");
    for number in 1..5 {
        println!("{}", number);
    }

    // 遍历包含上限的范围
    println!("\nRange 1..=5:");
    for number in 1..=5 {
        println!("{}", number);
    }

    // 反向范围
    println!("\nReverse range (1..4).rev():");
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // 遍历迭代器
    println!("\nIterating over a vector:");
    let fruits = vec!["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("Fruit {}: {}", index + 1, fruit);
    }

    println!("\n=== 6. match 表达式 ===");
    // 基本 match 表达式
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("Coin value: {} cents", value);

    // match 与 Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("plus_one(Some(5)) = {:?}", six);
    println!("plus_one(None) = {:?}", none);

    // match 中的模式绑定
    let message = Message::ChangeColor(255, 0, 0);
    match message {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    println!("\n=== 7. 控制流标签 ===");
    // 标签用于嵌套循环中指定要 break 或 continue 的循环
    let mut count = 0;
    'outer: loop {
        println!("Outer loop iteration {}", count);
        count += 1;
        
        let mut inner_count = 0;
        'inner: loop {
            println!("  Inner loop iteration {}", inner_count);
            inner_count += 1;
            
            if inner_count == 3 {
                continue 'outer; // 跳过外层循环的当前迭代
            }
            
            if count == 5 {
                break 'outer; // 跳出外层循环
            }
        }
    }

    println!("\n=== 8. 控制流与表达式 ===");
    // 在 Rust 中，几乎所有东西都是表达式
    let result = if condition() {
        calculate_value(10)
    } else {
        calculate_value(20)
    };
    println!("Expression result: {}", result);

    // 循环也可以作为表达式的一部分
    let loop_result = {
        let mut x = 0;
        loop {
            x += 1;
            if x == 3 {
                break x * 10;
            }
        }
    };
    println!("Loop as expression result: {}", loop_result);
}

// ===============================================================================
// 辅助函数和类型
// ===============================================================================

/// 模拟条件判断
fn condition() -> bool {
    true
}

/// 模拟值计算
fn calculate_value(x: i32) -> i32 {
    x * 2
}

// 枚举类型，用于 match 示例
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 州枚举，用于 Quarter 变体
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // 其他州...
}

/// 计算硬币价值
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!\n", state);
            25
        },
    }
}

/// 给 Option<i32> 加 1
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 消息枚举，用于模式绑定示例
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// ===============================================================================
// 与其他语言对比
// ===============================================================================

/*
对比其他语言：

- C/C++:
  - if、while、for 语法类似
  - 没有 if let
  - 没有 match 表达式（C++17 有 std::variant 和 std::visit）
  - 循环不能返回值

- Java:
  - if、while、for 语法类似
  - 有 switch 语句，但不如 match 强大
  - 没有 if let
  - 循环不能返回值

- Python:
  - if 语法类似，但使用缩进
  - for 循环更灵活，可直接遍历可迭代对象
  - 没有 match 表达式（Python 3.10+ 有 match-case）
  - 循环不能返回值

- JavaScript:
  - if、while、for 语法类似
  - 有 switch 语句
  - 有三元运算符（condition ? expr1 : expr2）
  - 循环不能返回值

Rust 的特点：
- if 是表达式，可返回值
- loop 是表达式，可返回值
- match 表达式强大，支持模式匹配
- if let 简化了单一模式的匹配
- 控制流标签用于嵌套循环
- 所有控制流结构都与表达式系统集成
*/
