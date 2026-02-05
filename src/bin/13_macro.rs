fn main() {
    // 使用 vec! 创建动态数组
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 使用 println! 打印数组长度
    println!("Array length: {}", numbers.len());
    
    // 使用 dbg! 打印数组内容并调试
    let doubled = dbg!(numbers.iter().map(|x| x * 2).collect::<Vec<_>>());
    
    // 使用 assert! 验证结果
    assert!(!doubled.is_empty(), "Doubled array should not be empty");
    
    // 打印最终结果
    println!("Doubled numbers: {:?}", doubled);
}