/// Rust 集合类型详解
/// 
/// 本文件演示了 Rust 标准库中各种集合类型的使用：
/// 1. Vec（动态数组）
/// 2. String（字符串）
/// 3. HashMap（哈希表）
/// 4. HashSet（哈希集合）
/// 5. BTreeMap（有序映射）
/// 6. BTreeSet（有序集合）
/// 7. 集合的性能特性和使用场景

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};

fn main() {
    println!("=== 1. Vec（动态数组）===");
    // Vec 是 Rust 中最常用的集合类型，用于存储同类型的元素
    // 动态数组，可以在运行时添加或删除元素
    
    // 方法 1：使用 Vec::new() 创建空 Vec
    let v: Vec<i32> = Vec::new();
    println!("Empty Vec: {:?}", v);
    
    // 方法 2：使用 vec! 宏创建 Vec
    let mut v2 = vec![1, 2, 3]; // 宏创建
    println!("Initial Vec: {:?}", v2);
    
    // 向 Vec 中添加元素
    v2.push(4);
    println!("After push: {:?}", v2);
    
    // 访问 Vec 中的元素
    // 方法 1：使用 [] 索引访问（如果索引越界会 panic!）
    let first = v2[0];
    println!("First element: {}", first);
    
    // 方法 2：使用 get 方法访问（返回 Option<T>，不会 panic!）
    match v2.get(100) {
        Some(val) => println!("val = {}", val),
        None => println!("No such element!"),
    }
    
    // 遍历 Vec
    println!("\nIterating over Vec:");
    for i in &v2 {
        println!("Element: {}", i);
    }
    
    // 遍历并修改 Vec
    println!("\nModifying elements while iterating:");
    for i in &mut v2 {
        *i *= 2;
    }
    println!("Modified Vec: {:?}", v2);
    
    // 移除 Vec 中的元素
    let removed = v2.pop();
    println!("Removed element: {:?}, Vec after pop: {:?}", removed, v2);
    
    // Vec 的长度和容量
    println!("\nVec length: {}, capacity: {}", v2.len(), v2.capacity());
    
    // Vec 切片
    let slice = &v2[1..3];
    println!("Slice: {:?}", slice);

    println!("\n=== 2. String（字符串）===");
    // String 是 Rust 中 UTF-8 编码的字符串类型
    // 动态字符串，可以在运行时修改
    
    // 方法 1：使用 String::from 创建字符串
    let mut s = String::from("foo");
    println!("Initial String: '{}'", s);
    
    // 向字符串中添加内容
    s.push_str("bar"); // 修改字符串
    println!("After push_str: '{}'", s);
    
    // 单个字符
    s.push('!');
    println!("After push: '{}'", s);
    
    // 字符串拼接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意：s1 被 move，s2 仅借用
    println!("Concatenated String: '{}'", s3);
    // println!("s1 is now invalid: '{}'", s1); // 编译错误：s1 已被 move
    
    // 使用 format! 宏拼接字符串
    let s4 = format!("{}{}", "Hello, ", "Rust!");
    println!("Formatted String: '{}'", s4);
    
    // 字符串长度（字节数）
    let s5 = String::from("你好，Rust!");
    println!("String: '{}', length: {} bytes", s5, s5.len());
    
    // 遍历字符串字符
    println!("\nIterating over characters:");
    for c in s5.chars() {
        println!("Character: '{}'", c);
    }
    
    // 字符串切片
    let s6 = String::from("Hello, world!");
    let slice = &s6[0..5];
    println!("String slice: '{}'", slice);

    println!("\n=== 3. HashMap（哈希表）===");
    // HashMap 是 Rust 中的哈希表，用于存储键值对
    // 键和值可以是任意类型，但键必须实现 Eq 和 Hash trait
    
    // 方法 1：使用 HashMap::new() 创建空 HashMap
    let mut scores = HashMap::new();
    
    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Initial HashMap: {:?}", scores);
    
    // 访问键值对
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score for Blue: {:?}", score); // Some(10)
    
    // 遍历 HashMap
    println!("\nIterating over HashMap:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // 更新 HashMap
    // 方法 1：覆盖已有键的值
    scores.insert(String::from("Blue"), 25);
    println!("After insert: {:?}", scores);
    
    // 方法 2：仅当键不存在时插入
    scores.entry(String::from("Blue")).or_insert(0); // 不会修改，因为 Blue 已存在
    scores.entry(String::from("Green")).or_insert(30); // 会插入，因为 Green 不存在
    println!("After entry: {:?}", scores);
    
    // 方法 3：基于旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("\nWord count: {:?}", map);

    println!("\n=== 4. HashSet（哈希集合）===");
    // HashSet 是 Rust 中的哈希集合，用于存储唯一的元素
    // 元素必须实现 Eq 和 Hash trait
    
    // 方法 1：使用 HashSet::new() 创建空 HashSet
    let mut set = HashSet::new();
    
    // 插入元素
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2); // 重复元素，不会被插入
    println!("Initial HashSet: {:?}", set);
    
    // 检查元素是否存在
    let contains = set.contains(&2);
    println!("Contains 2: {}", contains);
    
    // 移除元素
    let removed = set.remove(&2);
    println!("Removed 2: {}, HashSet after remove: {:?}", removed, set);
    
    // 遍历 HashSet
    println!("\nIterating over HashSet:");
    for i in &set {
        println!("Element: {}", i);
    }
    
    // 集合操作
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("\nIntersection: {:?}", intersection);
    
    // 并集
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union);
    
    // 差集
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Difference: {:?}", difference);

    println!("\n=== 5. BTreeMap（有序映射）===");
    // BTreeMap 是 Rust 中的有序映射，基于 B 树实现
    // 键会自动排序，键必须实现 Ord trait
    
    // 方法 1：使用 BTreeMap::new() 创建空 BTreeMap
    let mut map = BTreeMap::new();
    
    // 插入键值对
    map.insert(String::from("Banana"), 100);
    map.insert(String::from("Apple"), 200);
    map.insert(String::from("Cherry"), 50);
    println!("Initial BTreeMap: {:?}", map);
    
    // BTreeMap 会自动按键排序
    println!("\nIterating over BTreeMap (sorted):");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    // 范围查询
    println!("\nRange query (A to C):");
    for (key, value) in map.range(String::from("A")..String::from("C")) {
        println!("{}: {}", key, value);
    }

    println!("\n=== 6. BTreeSet（有序集合）===");
    // BTreeSet 是 Rust 中的有序集合，基于 B 树实现
    // 元素会自动排序，元素必须实现 Ord trait
    
    // 方法 1：使用 BTreeSet::new() 创建空 BTreeSet
    let mut set = BTreeSet::new();
    
    // 插入元素
    set.insert(3);
    set.insert(1);
    set.insert(2);
    set.insert(2); // 重复元素，不会被插入
    println!("Initial BTreeSet: {:?}", set);
    
    // BTreeSet 会自动按元素排序
    println!("\nIterating over BTreeSet (sorted):");
    for i in &set {
        println!("Element: {}", i);
    }
    
    // 范围查询
    println!("\nRange query (1 to 3):");
    for i in set.range(1..3) {
        println!("Element: {}", i);
    }

    println!("\n=== 7. 集合的性能特性和使用场景 ===");
    println!("\n1. Vec:");
    println!("   - 性能：随机访问 O(1)，末尾插入/删除 O(1)（均摊）");
    println!("   - 使用场景：存储同类型元素，需要频繁访问或修改");
    
    println!("\n2. String:");
    println!("   - 性能：随机访问 O(n)（因为 UTF-8 编码），末尾插入 O(1)（均摊）");
    println!("   - 使用场景：存储文本数据，需要处理 UTF-8 编码");
    
    println!("\n3. HashMap:");
    println!("   - 性能：插入、删除、查找 O(1)（平均）");
    println!("   - 使用场景：存储键值对，需要快速查找");
    
    println!("\n4. HashSet:");
    println!("   - 性能：插入、删除、查找 O(1)（平均）");
    println!("   - 使用场景：存储唯一元素，需要快速检查元素是否存在");
    
    println!("\n5. BTreeMap:");
    println!("   - 性能：插入、删除、查找 O(log n)");
    println!("   - 使用场景：存储键值对，需要有序遍历或范围查询");
    
    println!("\n6. BTreeSet:");
    println!("   - 性能：插入、删除、查找 O(log n)");
    println!("   - 使用场景：存储唯一元素，需要有序遍历或范围查询");
    
    println!("\n=== 8. 集合的选择建议 ===");
    println!("   - 如果需要随机访问元素，选择 Vec");
    println!("   - 如果需要存储文本，选择 String");
    println!("   - 如果需要存储键值对且不需要有序，选择 HashMap");
    println!("   - 如果需要存储键值对且需要有序，选择 BTreeMap");
    println!("   - 如果需要存储唯一元素且不需要有序，选择 HashSet");
    println!("   - 如果需要存储唯一元素且需要有序，选择 BTreeSet");
}