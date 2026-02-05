/// 泛型：编写适用于多种类型的代码
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// Trait：定义共享行为（类似 Java interface）
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // 默认实现
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    // 使用默认实现
}

// 泛型函数使用 trait bound
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::从("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
}