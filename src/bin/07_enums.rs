/// 枚举：可包含多种不同类型的数据（比 Java enum 强大得多）
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    // ===== Option 枚举（替代 null）=====
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    // 必须处理 Some 和 None
    match some_number {
        Some(x) => println!("Got {}", x),
        None => println!("No number"),
    }

    // ===== match 表达式 =====
    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }

    // ===== if let 简化 match =====
    if let Some(x) = some_number {
        println!("Got {}", x);
    }
}