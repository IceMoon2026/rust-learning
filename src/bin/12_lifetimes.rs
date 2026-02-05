/// 生命周期：确保引用始终有效（防止悬垂引用）
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 'a 是生命周期参数，表示两个输入和输出有相同生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ===== 结构体中的生命周期 =====
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main2() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
}

// 🆚 对比 C++：
// - C++ 用 RAII 管理资源，但引用可能悬垂
// - Rust 编译器通过生命周期分析在编译期阻止悬垂引用