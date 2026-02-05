/// 所有权规则：
/// 1. 每个值有唯一所有者
/// 2. 所有者离开作用域时，值被 drop
/// 3. 值可被移动（move）或借用（borrow）

fn main() {
    // ===== 移动（Move）=====
    let s1 = String::from("hello");
    let s2 = s1; // s1 被 move 到 s2，s1 无效！
    // println!("{}", s1); // ❌ 编译错误：value borrowed here after move

    // ===== 克隆（Clone）=====
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝
    println!("s1 = {}, s2 = {}", s1, s2); // OK

    // ===== 借用（Borrowing）=====
    let s = String::from("hello");
    takes_ref(&s); // 传递引用（不获取所有权）
    println!("{}", s); // s 仍有效！

    // ===== 可变借用 =====
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // hello, world!

    // ❌ 不能同时有可变和不可变引用
    // let r1 = &s;
    // let r2 = &mut s; // ❌ 编译错误
}

fn takes_ref(s: &String) {
    println!("Got: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// 🆚 对比 Java：
// - Java 所有对象都是引用（类似 Rust 的 &T），但有 GC
// - Rust 无 GC，靠编译器检查引用生命周期