use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // ===== panic! =====
    // panic!("crash and burn"); // 程序终止

    // ===== Result 枚举 =====
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };

    // ===== ? 操作符（简化错误传播）=====
    let f2 = open_file("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap()
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    // 在返回 Result 的函数中使用 ?
    let _ = read_username_from_file();
}

fn open_file(filename: &str) -> Result<File, std::io::Error> {
    let f = File::open(filename)?;
    Ok(f)
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 🆚 对比 Java：
// - Java 用 try/catch，Rust 用 Result + match/?
// - Rust 强制处理错误，无 unchecked exception