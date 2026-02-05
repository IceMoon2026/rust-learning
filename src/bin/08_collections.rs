use std::collections::HashMap;

fn main() {
    // ===== Vec（动态数组）=====
    let v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3]; // 宏创建
    v2.push(4);
    // 访问：v2[0] 或 v2.get(0)
    match v2.get(100) {
        Some(val) => println!("val = {}", val),
        None => println!("No such element!"),
    }

    // ===== String =====
    let mut s = String::from("foo");
    s.push_str("bar"); // 修改字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意：s1 被 move，s2 仅借用

    // ===== HashMap =====
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 访问
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score); // Some(10)

    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新策略
    scores.entry(String::from("Blue")).or_insert(0); // 如果不存在才插入
}