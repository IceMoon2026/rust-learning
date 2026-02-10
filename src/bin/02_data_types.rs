/// Rust 是静态类型语言，但通常能自动推断类型
/// 
/// 本文件演示了 Rust 中的数据类型，包括：
/// 1. 标量类型（Scalar Types）
///    - 整数类型
///    - 浮点数类型
///    - 布尔类型
///    - 字符类型
/// 2. 复合类型（Compound Types）
///    - 元组（Tuple）
///    - 数组（Array）
/// 3. 类型转换
/// 4. 类型大小和范围

fn main() {
    println!("=== 1. 标量类型（Scalar Types）===");
    println!("标量类型代表单个值，Rust 有四种主要的标量类型：");

    println!("\n--- 1.1 整数类型 ---");
    // 整数类型：有符号（i）和无符号（u），长度从 8 位到 128 位
    // 默认整数类型：i32（平衡性能和大小）
    
    // 有符号整数（可表示正负）
    let a: i8 = -128;    // 范围：-128 到 127
    let b: i16 = 32767;  // 范围：-32768 到 32767
    let c: i32 = -2_147_483_648; // 范围：-2^31 到 2^31-1
    let d: i64 = 9_223_372_036_854_775_807; // 范围：-2^63 到 2^63-1
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    
    // 无符号整数（只能表示非负）
    let f: u8 = 255;     // 范围：0 到 255
    let g: u16 = 65535;  // 范围：0 到 65535
    let h: u32 = 4_294_967_295; // 范围：0 到 2^32-1
    let i: u64 = 18_446_744_073_709_551_615; // 范围：0 到 2^64-1
    let j: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    
    // 架构相关的整数类型
    let k: isize = 100;  // 与指针同大小，32位系统为 i32，64位系统为 i64
    let l: usize = 200;  // 与指针同大小，用于索引
    
    println!("i8: {}, 范围: -128 到 127", a);
    println!("u8: {}, 范围: 0 到 255", f);
    println!("i32 (默认): {}, 范围: -2^31 到 2^31-1", c);
    println!("usize (架构相关): {}, 用于数组索引等", l);

    println!("\n--- 1.2 浮点数类型 ---");
    // 浮点数类型：f32（单精度）和 f64（双精度，默认）
    let x: f32 = 3.14;    // 单精度，32 位
    let y: f64 = 2.71828; // 双精度，64 位（默认）
    
    // 浮点数运算：需要相同类型
    let sum_f32 = x + (y as f32); // 将 f64 转换为 f32
    let sum_f64 = (x as f64) + y; // 将 f32 转换为 f64
    let product_f32 = x * (y as f32); // 将 f64 转换为 f32
    let product_f64 = (x as f64) * y; // 将 f32 转换为 f64
    
    println!("f32: {}", x);
    println!("f64 (默认): {}", y);
    println!("sum (f32): {}", sum_f32);
    println!("sum (f64): {}", sum_f64);
    println!("product (f32): {}", product_f32);
    println!("product (f64): {}", product_f64);

    println!("\n--- 1.3 布尔类型 ---");
    // 布尔类型：true 或 false，占用 1 字节
    let is_true: bool = true;
    let is_false: bool = false;
    
    // 布尔运算
    let and_result = is_true && is_false; // 逻辑与
    let or_result = is_true || is_false;  // 逻辑或
    let not_result = !is_true;            // 逻辑非
    
    println!("true: {}", is_true);
    println!("false: {}", is_false);
    println!("true && false: {}", and_result);
    println!("true || false: {}", or_result);
    println!("!true: {}", not_result);

    println!("\n--- 1.4 字符类型 ---");
    // 字符类型：Unicode 标量值，占用 4 字节
    let char_a: char = 'a';      // ASCII 字符
    let char_greek: char = 'α';   // 希腊字母
    let char_chinese: char = '中'; // 中文字符
    let char_emo: char = '😀';    // 表情符号
    let char_special: char = '\u{1F600}'; // Unicode 码点
    
    println!("ASCII: '{}'", char_a);
    println!("希腊字母: '{}'", char_greek);
    println!("中文: '{}'", char_chinese);
    println!("表情: '{}'", char_emo);
    println!("Unicode: '{}'", char_special);

    println!("\n=== 2. 复合类型（Compound Types）===");
    println!("复合类型可以组合多个值：");

    println!("\n--- 2.1 元组（Tuple）---");
    // 元组：固定长度，可包含不同类型的值
    let tuple: (i32, f64, char, bool) = (500, 6.4, 'a', true);
    
    // 元组解构
    let (int_val, float_val, char_val, bool_val) = tuple;
    println!("Tuple elements: {}, {}, '{}', {}", int_val, float_val, char_val, bool_val);
    
    // 通过索引访问元组元素
    println!("First element: {}", tuple.0);
    println!("Second element: {}", tuple.1);
    println!("Third element: '{}'", tuple.2);
    println!("Fourth element: {}", tuple.3);
    
    // 空元组（单元类型）
    let unit: () = ();
    println!("Unit type: {:?}", unit);

    println!("\n--- 2.2 数组（Array）---");
    // 数组：固定长度，所有元素类型相同
    // 数组类型格式：[类型; 长度]
    
    // 初始化方式 1：列出所有元素
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    
    // 初始化方式 2：重复值
    let arr2: [u8; 4] = [0; 4]; // [0, 0, 0, 0]
    
    // 访问数组元素
    println!("arr1[0]: {}", arr1[0]);
    println!("arr1[2]: {}", arr1[2]);
    println!("arr2[1]: {}", arr2[1]);
    
    // 数组长度
    println!("arr1 length: {}", arr1.len());
    println!("arr2 length: {}", arr2.len());
    
    // 数组切片（引用数组的一部分）
    let slice: &[i32] = &arr1[1..4]; // 包含索引 1, 2, 3 的元素
    println!("Slice elements: {:?}", slice);

    println!("\n=== 3. 类型转换 ===");
    // 显式类型转换（Rust 不会自动转换类型）
    let int_val: i32 = 100;
    let float_val: f64 = int_val as f64;
    let unsigned_val: u32 = int_val as u32;
    
    println!("i32 to f64: {} -> {}", int_val, float_val);
    println!("i32 to u32: {} -> {}", int_val, unsigned_val);
    
    // 注意：类型转换可能导致数据丢失
    let large_int: i32 = 1000;
    let small_uint: u8 = large_int as u8; // 1000 超出 u8 范围（0-255）
    println!("i32 (1000) to u8: {}", small_uint); // 结果：232（1000 % 256）

    println!("\n=== 4. 类型大小和内存布局 ===");
    // 使用 std::mem::size_of 查看类型大小（字节）
    println!("Size of i8: {} bytes", std::mem::size_of::<i8>());
    println!("Size of i32: {} bytes", std::mem::size_of::<i32>());
    println!("Size of i64: {} bytes", std::mem::size_of::<i64>());
    println!("Size of f32: {} bytes", std::mem::size_of::<f32>());
    println!("Size of f64: {} bytes", std::mem::size_of::<f64>());
    println!("Size of bool: {} bytes", std::mem::size_of::<bool>());
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
    println!("Size of tuple (i32, f64): {} bytes", std::mem::size_of::<(i32, f64)>());
    println!("Size of array [i32; 5]: {} bytes", std::mem::size_of::<[i32; 5]>());

    println!("\n=== 5. 类型推断 ===");
    // Rust 通常能自动推断类型
    let inferred_int = 42;         // 推断为 i32
    let inferred_float = 3.14;      // 推断为 f64
    let inferred_bool = true;       // 推断为 bool
    let inferred_char = 'x';        // 推断为 char
    
    // 可以通过类型注解覆盖推断
    let explicit_uint: u32 = 100;   // 显式指定为 u32
    let explicit_float: f32 = 2.5;  // 显式指定为 f32
    
    println!("Inferred types:");
    println!("42 -> i32");
    println!("3.14 -> f64");
    println!("true -> bool");
    println!("'x' -> char");
    println!("Explicit types:");
    println!("100u32 -> u32");
    println!("2.5f32 -> f32");

    println!("\n=== 6. 与其他语言对比 ===");
    println!("- C/C++: 类似的类型系统，但 Rust 更安全（无未定义行为）");
    println!("- Java: Rust 有更多整数类型（如 i8, u16 等），Java 只有 int 和 long");
    println!("- Python: Rust 是静态类型，Python 是动态类型");
    println!("- JavaScript: Rust 类型更严格，JS 会自动转换类型");
    println!("- Go: 类似的类型系统，但 Rust 有更强大的类型推断");
}
