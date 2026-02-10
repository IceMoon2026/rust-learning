/// Rust 函数系统详解
/// 
/// 本文件演示了 Rust 中函数的各种特性：
/// 1. 函数定义和调用
/// 2. 参数类型和返回值
/// 3. 表达式与语句
/// 4. 递归函数
/// 5. 高阶函数（函数作为参数）
/// 6. 闭包
/// 7. 函数指针
/// 8. 不同类型的函数

fn main() {
    println!("=== 1. 基本函数定义和调用 ===");
    // 调用无参数函数
    greet();
    
    // 调用带参数函数
    print_number(42);
    
    // 调用带多个参数函数
    add_and_print(10, 20);

    println!("\n=== 2. 函数返回值 ===");
    // 调用有返回值的函数
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    // 调用返回单元类型的函数
    let unit_result = no_return_value();
    println!("Unit type result: {:?}", unit_result);

    println!("\n=== 3. 表达式 vs 语句 ===");
    // 语句：执行操作但不返回值，以分号结尾
    let a = 5; // 语句
    println!("a = {}", a); // 语句
    
    // 表达式：执行操作并返回值，不加分号
    let b = {
        let x = 3;
        x + 1 // 表达式，返回 4
    };
    println!("b = {}", b); // 输出 4
    
    // 函数体本身也是一个表达式
    let c = calculate();
    println!("c = {}", c); // 输出 10

    println!("\n=== 4. 递归函数 ===");
    // 计算阶乘
    let factorial_result = factorial(5);
    println!("5! = {}", factorial_result);
    
    // 计算斐波那契数列
    let fib_result = fibonacci(10);
    println!("Fibonacci(10) = {}", fib_result);

    println!("\n=== 5. 高阶函数 ===");
    // 函数作为参数
    let numbers = [1, 2, 3, 4, 5];
    println!("Original numbers: {:?}", numbers);
    
    // 传递函数作为参数
    let doubled = apply_to_all(&numbers, double);
    println!("Doubled numbers: {:?}", doubled);
    
    let squared = apply_to_all(&numbers, square);
    println!("Squared numbers: {:?}", squared);

    println!("\n=== 6. 闭包 ===");
    // 闭包：匿名函数，可以捕获环境变量
    let factor = 3;
    let triple = |x| x * factor;
    println!("Triple of 5: {}", triple(5));
    
    // 闭包作为参数
    let tripled = apply_to_all_with_closure(&numbers, |&x| x * 3);
    println!("Tripled numbers: {:?}", tripled);

    println!("\n=== 7. 函数指针 ===");
    // 函数指针：指向函数的指针
    let add_fn: fn(i32, i32) -> i32 = add;
    let sum = add_fn(10, 20);
    println!("10 + 20 = {}", sum);

    println!("\n=== 8. 函数重载（Rust 不支持）===");
    println!("Rust 不支持传统意义上的函数重载，但可以通过以下方式实现类似功能：");
    println!("- 使用不同的函数名");
    println!("- 使用泛型");
    println!("- 使用 trait 实现");
    
    // 示例：使用不同函数名
    println!("add_i32(5, 3) = {}", add_i32(5, 3));
    println!("add_f64(2.5, 3.5) = {}", add_f64(2.5, 3.5));

    println!("\n=== 9. 函数命名约定 ===");
    println!("Rust 函数命名使用蛇形命名法（snake_case）：");
    println!("- 小写字母");
    println!("- 单词之间用下划线分隔");
    println!("- 例如：add_numbers, calculate_factorial");
}

// ===============================================================================
// 1. 基本函数
// ===============================================================================

/// 无参数、无返回值的函数
fn greet() {
    println!("Hello, Rust!");
}

/// 带参数的函数
/// 参数必须标注类型
fn print_number(num: i32) {
    println!("The number is: {}", num);
}

/// 带多个参数的函数
fn add_and_print(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// ===============================================================================
// 2. 函数返回值
// ===============================================================================

/// 带返回值的函数
/// 返回值类型用 -> 声明
fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式作为返回值，不加分号
}

/// 返回单元类型（无返回值）
/// 单元类型用 () 表示，类似于其他语言的 void
fn no_return_value() -> () {
    println!("This function returns nothing");
    // 可以不显式返回，隐式返回 ()
}

/// 省略返回类型注解（仅当返回 () 时）
fn implicit_unit_return() {
    println!("This function also returns ()");
}

/// 函数体是表达式
fn calculate() -> i32 {
    let x = 5;
    let y = 5;
    x + y // 表达式作为返回值
}

// ===============================================================================
// 3. 递归函数
// ===============================================================================

/// 递归计算阶乘
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// 递归计算斐波那契数列
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// ===============================================================================
// 4. 高阶函数
// ===============================================================================

/// 应用函数到数组的每个元素
fn apply_to_all(numbers: &[i32], func: fn(i32) -> i32) -> Vec<i32> {
    numbers.iter().map(|&x| func(x)).collect()
}

/// 双倍数函数
fn double(x: i32) -> i32 {
    x * 2
}

/// 平方函数
fn square(x: i32) -> i32 {
    x * x
}

/// 应用闭包到数组的每个元素
fn apply_to_all_with_closure<F>(numbers: &[i32], func: F) -> Vec<i32>
where
    F: Fn(&i32) -> i32,
{
    numbers.iter().map(func).collect()
}

// ===============================================================================
// 5. 不同类型的函数示例
// ===============================================================================

/// 整数加法函数
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

/// 浮点数加法函数
fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

// ===============================================================================
// 6. 与其他语言对比
// ===============================================================================

/*
对比其他语言：

- C/C++:
  - 函数声明和定义分离
  - 支持函数重载
  - 支持默认参数
  - 无闭包（C++11 引入 lambda）

- Java:
  - 函数称为方法，必须在类中定义
  - 支持方法重载
  - 支持默认参数（Java 8+）
  - 支持 lambda 表达式（Java 8+）

- Python:
  - 函数定义使用 def 关键字
  - 支持默认参数
  - 支持关键字参数
  - 支持可变参数
  - 支持 lambda 表达式

- JavaScript:
  - 函数定义使用 function 关键字或箭头函数
  - 支持默认参数
  - 支持剩余参数
  - 支持箭头函数
  - 函数是一等公民

Rust 的特点：
- 函数是独立的，不需要在类中定义
- 没有函数重载，但可以使用泛型
- 没有默认参数，但可以使用 Option 类型
- 支持闭包
- 支持高阶函数
- 表达式作为返回值
- 单元类型 () 代替 void
*/
