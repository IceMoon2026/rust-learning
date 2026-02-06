/// 枚举（Enum）：Rust 中强大的类型系统特性
/// 
/// Rust 的枚举比其他语言（如 Java）的枚举更强大，因为：
/// 1. 每个变体可以包含不同类型和数量的数据
/// 2. 可以为枚举实现方法
/// 3. 可以与模式匹配（pattern matching）配合使用
/// 4. 用于创建类型安全的状态机

// 示例 1: 基本枚举 - IP 地址类型
#[derive(Debug)]
enum IpAddr {
    // 变体 V4 包含一个 String 类型的数据
    V4(String),
    // 变体 V6 包含一个 String 类型的数据
    V6(String),
}

// 示例 2: 复杂枚举 - 消息类型
#[derive(Debug)]
enum Message {
    // 无数据的变体
    Quit,
    // 包含匿名结构体的变体
    Move { x: i32, y: i32 },
    // 包含单个 String 类型的变体
    Write(String),
    // 包含三个 i32 类型的变体
    ChangeColor(i32, i32, i32),
}

// 为枚举实现方法
impl Message {
    /// 为 Message 枚举实现 call 方法
    fn call(&self) {
        // 使用 match 表达式处理不同变体
        match self {
            Message::Quit => println!("Message::Quit called"),
            Message::Move { x, y } => println!("Message::Move called with x={}, y={}", x, y),
            Message::Write(text) => println!("Message::Write called with text={}", text),
            Message::ChangeColor(r, g, b) => println!("Message::ChangeColor called with r={}, g={}, b={}", r, g, b),
        }
    }
}

// 示例 3: 嵌套枚举 - 更复杂的数据结构
#[derive(Debug)]
enum Device {
    Laptop {
        brand: String,
        model: String,
    },
    Smartphone {
        os: OperatingSystem,
        storage: u32, // 存储容量（GB）
    },
}

// 嵌套的操作系统枚举
#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
    iOS,
    Android,
}

// 示例 4: 枚举与 trait 实现
#[derive(Debug)]
enum Shape {
    Circle(f64),      // 半径
    Square(f64),      // 边长
    Rectangle(f64, f64), // 长和宽
}

// 为 Shape 实现计算面积的方法
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(length, width) => length * width,
        }
    }
}

fn main() {
    println!("===== 基本枚举使用 =====");
    // 创建 IpAddr 枚举的实例
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home IP: {:?}, Loopback IP: {:?}", home, loopback);

    println!("\n===== 枚举方法调用 =====");
    // 创建 Message 枚举的不同实例并调用方法
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    println!("\n===== Option 枚举（替代 null）=====");
    // Option 是 Rust 标准库中的枚举，用于表示可能存在或不存在的值
    // Option<T> 有两个变体：Some(T) 和 None
    
    let some_number = Some(5);          // 类型自动推断为 Option<i32>
    let some_string = Some("Hello");    // 类型自动推断为 Option<&str>
    let absent_number: Option<i32> = None; // 必须显式指定类型
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    println!("\n===== match 表达式完整匹配 =====");
    // match 表达式必须覆盖所有可能的变体
    match some_number {
        Some(x) => println!("Got number: {}", x),
        None => println!("No number provided"),
    }

    // 匹配 Message 枚举
    let msg = Message::Write(String::from("Rust is awesome!"));
    match msg {
        Message::Quit => println!("Action: Quit"),
        Message::Move { x, y } => println!("Action: Move to ({}, {})", x, y),
        Message::Write(text) => println!("Action: Write '{}'", text),
        Message::ChangeColor(r, g, b) => println!("Action: Change color to RGB({}, {}, {})", r, g, b),
    }

    println!("\n===== if let 简化匹配 =====");
    // 当只关心一种变体时，使用 if let 比 match 更简洁
    if let Some(x) = some_number {
        println!("if let: Got number {}", x);
    } else {
        println!("if let: No number");
    }

    // 另一个 if let 示例
    let color = Message::ChangeColor(0, 255, 0);
    if let Message::ChangeColor(r, g, b) = color {
        println!("if let: Changing color to RGB({}, {}, {})", r, g, b);
    }

    println!("\n===== 嵌套枚举示例 =====");
    // 创建嵌套枚举实例
    let laptop = Device::Laptop {
        brand: String::from("Apple"),
        model: String::from("MacBook Pro"),
    };
    
    let smartphone = Device::Smartphone {
        os: OperatingSystem::iOS,
        storage: 256,
    };
    
    println!("Device 1: {:?}", laptop);
    println!("Device 2: {:?}", smartphone);

    println!("\n===== 枚举与计算 =====");
    // 使用 Shape 枚举计算面积
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    
    println!("Circle area: {:.2}", circle.area());
    println!("Square area: {:.2}", square.area());
    println!("Rectangle area: {:.2}", rectangle.area());

    println!("\n===== Result 枚举（错误处理）=====");
    // Result 是 Rust 标准库中的枚举，用于表示可能失败的操作
    // Result<T, E> 有两个变体：Ok(T) 表示成功，Err(E) 表示失败
    
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(error) => println!("Error: {}", error),
    }
    
    match divide(5.0, 0.0) {
        Ok(result) => println!("5.0 / 0.0 = {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
