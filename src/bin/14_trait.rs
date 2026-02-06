/// Trait（特质）：Rust 中的接口系统
/// 
/// Trait 是 Rust 中实现代码复用和多态的核心机制，类似于其他语言中的接口（interface）。
/// 它定义了一组方法签名，类型可以实现这些方法来提供特定的行为。

// ===============================================================================
// 示例 1: 基本 Trait 定义
// ===============================================================================

/// 定义一个 `Summary` trait，包含一个 `summarize` 方法
pub trait Summary {
    /// 返回一个摘要字符串
    fn summarize(&self) -> String;
}

// ===============================================================================
// 示例 2: 为类型实现 Trait
// ===============================================================================

/// 新闻文章结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// 为 `NewsArticle` 实现 `Summary` trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

/// 推文结构体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/// 为 `Tweet` 实现 `Summary` trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ===============================================================================
// 示例 3: Trait 方法的默认实现
// ===============================================================================

/// 定义一个带有默认实现的 `DefaultSummary` trait
pub trait DefaultSummary {
    /// 返回一个摘要字符串（默认实现）
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

/// 为 `NewsArticle` 实现 `DefaultSummary` trait（使用默认实现）
impl DefaultSummary for NewsArticle {
    // 这里没有重写 summarize 方法，将使用默认实现
}

/// 为 `Tweet` 实现 `DefaultSummary` trait（重写默认实现）
impl DefaultSummary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} (custom default)", self.username, self.content)
    }
}

// ===============================================================================
// 示例 4: Trait 作为参数
// ===============================================================================

/// 接受任何实现了 `Summary` trait 的类型
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// 使用 trait 约束的语法（与上面等价）
fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// 多个 trait 约束（使用 + 号）
fn notify_multiple<T: Summary + Default>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// 使用 where 子句简化多个 trait 约束
fn notify_where<T>(item: &T) where T: Summary + Default {
    println!("Breaking news! {}", item.summarize());
}

// ===============================================================================
// 示例 5: Trait 作为返回类型
// ===============================================================================

/// 返回一个实现了 `Summary` trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 注意：返回 impl Trait 时，所有可能的返回值必须是同一类型
// 下面的函数会编译失败，因为它可能返回两种不同类型
/*
fn returns_summarizable_condition(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/

// ===============================================================================
// 示例 6: 关联类型（Associated Types）
// ===============================================================================

/// 定义一个带有关联类型的 `Iterator` trait（简化版）
pub trait Iterator {
    /// 关联类型，表示迭代器产生的元素类型
    type Item;
    
    /// 下一个元素
    fn next(&mut self) -> Option<Self::Item>;
}

/// 实现一个简单的计数器迭代器
struct Counter {
    count: u32,
    max: u32,
}

/// 为 `Counter` 实现 `Iterator` trait
impl Iterator for Counter {
    // 指定关联类型 `Item` 为 `u32`
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let current = self.count;
            self.count += 1;
            Some(current)
        } else {
            None
        }
    }
}

// ===============================================================================
// 示例 7: Derive Trait（派生特质）
// ===============================================================================

/// 使用 #[derive] 属性自动实现常见的 trait
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// ===============================================================================
// 示例 8: 特质继承
// ===============================================================================

/// 定义一个 `Drawable` trait
pub trait Drawable {
    fn draw(&self);
}

/// 定义一个 `Clickable` trait，继承自 `Drawable`
pub trait Clickable: Drawable {
    fn click(&self);
}

/// 按钮结构体
struct Button {
    label: String,
}

/// 为 `Button` 实现 `Drawable` trait
impl Drawable for Button {
    fn draw(&self) {
        println!("Drawing button: {}", self.label);
    }
}

/// 为 `Button` 实现 `Clickable` trait
impl Clickable for Button {
    fn click(&self) {
        println!("Clicking button: {}", self.label);
    }
}

// ===============================================================================
// 主函数：测试所有示例
// ===============================================================================

fn main() {
    println!("=== 示例 1-2: 基本 Trait 定义和实现 ===");
    let article: NewsArticle = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("Article summary: {}", Summary::summarize(&article));
    println!("Tweet summary: {}", Summary::summarize(&tweet));
    
    println!("\n=== 示例 3: 默认实现 ===");
    println!("Article default summary: {}", <NewsArticle as DefaultSummary>::summarize(&article));
    println!("Tweet default summary: {}", <Tweet as DefaultSummary>::summarize(&tweet));
    
    println!("\n=== 示例 4: Trait 作为参数 ===");
    notify(&article);
    notify_generic(&tweet);
    
    println!("\n=== 示例 5: Trait 作为返回类型 ===");
    let summarizable = returns_summarizable();
    println!("Returned summarizable: {}", summarizable.summarize());
    
    println!("\n=== 示例 6: 关联类型 ===");
    let mut counter = Counter { count: 0, max: 5 };
    println!("Counter values:");
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }
    
    println!("\n=== 示例 7: 派生 Trait ===");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };
    println!("p1: {:?}", p1);
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);
    
    println!("\n=== 示例 8: 特质继承 ===");
    let button = Button { label: String::from("Submit") };
    button.draw();
    button.click();
    
    println!("\n=== Trait 要点总结 ===");
    println!("1. Trait 定义了类型可以实现的行为接口");
    println!("2. 类型可以实现多个 Trait");
    println!("3. Trait 可以有默认方法实现");
    println!("4. Trait 可以作为参数类型和返回类型");
    println!("5. Trait 约束用于限制泛型类型");
    println!("6. 关联类型使 Trait 更加灵活");
    println!("7. 派生 Trait 提供了常见行为的自动实现");
    println!("8. Trait 可以继承其他 Trait");
}
