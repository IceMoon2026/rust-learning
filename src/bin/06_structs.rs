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
#[derive(Debug)]
struct Point(i32, i32, i32);

// 单元结构体（Unit-like Struct）
// 定义不同的状态（单元结构体）
#[derive(Debug)]
struct Ready;
#[derive(Debug)]
struct Running;
#[derive(Debug)]
struct Paused;

// 任务结构体，状态由泛型参数标记
#[derive(Debug)]
struct Task<S> {
    name: String,
    state: std::marker::PhantomData<S>, //  PhantomData 用于持有类型标记
}

// 为不同状态实现方法
impl Task<Ready> {
    fn new(name: &str) -> Self {
        Task {
            name: name.to_string(),
            state: std::marker::PhantomData,
        }
    }
    
    // 从 Ready 状态切换到 Running 状态
    fn start(self) -> Task<Running> {
        println!("Starting task: {}", self.name);
        Task {
            name: self.name,
            state: std::marker::PhantomData,
        }
    }
}

impl Task<Running> {
    // 从 Running 状态切换到 Paused 状态
    fn pause(self) -> Task<Paused> {
        println!("Pausing task: {}", self.name);
        Task {
            name: self.name,
            state: std::marker::PhantomData,
        }
    }
}



fn test_unit_struct() {
    let task = Task::new("Backup");
    let task = task.start(); // 只能在 Ready 状态调用 start
    let task = task.pause(); // 只能在 Running 状态调用 pause
    println!("task: {:?}", task);
    // task.start(); // 编译错误：Task<Paused> 没有 start 方法
}

impl User {
    /// 关联函数（associated function）:
    /// - 定义时没有 `self` 参数。
    /// - 通过 `Type::function()` 调用（使用 `::`）。
    /// - 常用于构造器/工厂方法（比如 `new`、`from_*`）。
    /// - 因为没有 `self`，不能访问实例字段，只有类型级别的逻辑。
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    /// 另一个关联函数示例：从 email 生成 User（演示命名习惯）
    fn from_email(email: &str) -> User {
        let username = email.split('@').next().unwrap_or("unknown").to_string();
        User::new(email.to_string(), username)
    }

    /// 方法（第一个参数是 self）
    /// 方法（method）：第一个参数是 `self`（有三种形式）
    /// - `self`：方法获取所有权，调用后原值被移动。
    /// - `&self`：通过不可变引用读取，不修改所有者。
    /// - `&mut self`：通过可变引用修改实例，需要 `mut` 绑定。
    /// 方法通过实例用点运算调用（`instance.method()`）。
    fn promote(&mut self) {
        self.sign_in_count += 1;
    }

    /// 激活用户账号
    fn activate(&mut self) {
        self.active = true;
    }

    /// 停用用户账号
    fn deactivate(&mut self) {
        self.active = false;
    }

    /// 检查用户是否激活
    fn is_active(&self) -> bool {
        self.active
    }

    /// 不可变引用方法示例：返回 email 的域名（不改变实例）
    fn email_domain(&self) -> Option<&str> {
        self.email.split('@').nth(1)
    }

    /// 通过 `self` 获取所有权的示例：消费自己并返回用户名
    fn into_username(self) -> String {
        self.username
    }
}

fn main() {
    // 测试单元结构体
    println!("test_unit_struct:");
    test_unit_struct();

    // 测试用户结构体
    println!("\n\n");
    println!("test_user:");
    
    let mut user1 = User::new(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    println!("user1 active status: {}", user1.is_active());
    
    user1.promote();
    println!("after promote: {:?}", user1);

    // 使用不可变方法读取信息（不会消耗所有权）
    if let Some(domain) = user1.email_domain() {
        println!("email domain: {}", domain);
    }

    // 演示停用和重新激活用户
    user1.deactivate();
    println!("after deactivate: {:?}", user1);
    println!("user1 active status: {}", user1.is_active());
    
    user1.activate();
    println!("after activate: {:?}", user1);
    println!("user1 active status: {}", user1.is_active());

    // 使用 by-value 方法会移动（消费）实例
    let username = user1.into_username();
    println!("moved out username: {}", username);
    // println!("try to use user1, {:?}", user1);
    // 此处不能再使用 user1，因为它已被移动到 into_username()

    // 使用关联函数构造另一个实例
    println!("\n\n");
    println!("test_user_from_email:");
    
    let user2 = User::from_email("alice@example.org");
    println!("user2: {:?}", user2);
    println!("user2 active status: {}", user2.is_active());


    // 测试元组结构体
    println!("\n\n");
    println!("test_tuple_struct:");

    let black: Color = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black = {:?}, origin = {:?}", black, origin);
    println!("black.0 = {}", black.0);
    println!("black.1 = {}", black.1);
    println!("black.2 = {}", black.2);
    println!("origin.0 = {}", origin.0);
    println!("origin.1 = {}", origin.1);
    println!("origin.2 = {}", origin.2);
    
    // 关联函数与方法的要点总结（简明）:
    // - 调用方式：关联函数 `Type::fn(...)`，方法 `value.fn(...)`。
    // - 是否有 `self`：关联函数没有 `self`，方法第一个参数必须是 `self`/`&self`/`&mut self`。
    // - 访问字段：只有方法（有 `self`）可以访问或修改实例字段；关联函数不能直接访问实例字段。
    // - 用途：关联函数常作构造器/工厂/辅助函数；方法用于与实例交互（读/写/消费）。
}