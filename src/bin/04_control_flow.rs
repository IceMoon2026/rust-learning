fn main() {
    // ===== if 表达式 =====
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 可作为表达式（必须有 else，且分支类型一致）
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);

    // ===== loop 循环 =====
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // loop 可返回值
        }
    };
    println!("result = {}", result); // 20

    // ===== while =====
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // ===== for in =====
    let a = [10, 20, 30];
    for element in a.iter() {
        println!("value: {}", element);
    }

    // 反向范围
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}