/// 结构体：自定义数据类型（类似 Java class，但无方法继承）
#[derive(Debug)] // 自动实现 Debug trait，方便打印
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体（Tuple Struct）—— 有名字段但无名称
#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 单元结构体（Unit-like Struct）
struct AlwaysEqual;

impl User {
    /// 关联函数（类似 Java static 方法）
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    /// 方法（第一个参数是 self）
    fn promote(&mut self) {
        self.sign_in_count += 1;
    }
}

fn main() {
    let mut user1 = User::new(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    user1.promote();
    println!("{:?}", user1);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black = {:?}, origin = {:?}", black, origin);
}