/// Rust 模块系统详解
/// 
/// 本文件演示了 Rust 中的模块系统，包括：
/// 1. 模块定义
/// 2. 路径（绝对路径和相对路径）
/// 3. use 声明
/// 4. 可见性（pub 关键字）
/// 5. 嵌套模块
/// 6. 文件系统模块
/// 7. 模块的使用场景

// ===============================================================================
// 1. 模块定义
// ===============================================================================

// 定义一个模块
mod front_of_house {
    // 模块内部的函数默认是私有的
    fn seat_at_table() {
        println!("Seated at table!");
    }
    
    // 定义子模块
    pub mod hosting {
        // 使用 pub 关键字使函数对外可见
        pub fn add_to_waitlist() {
            println!("Added to waitlist!");
        }
        
        // 子模块内部的函数默认是私有的
        fn seat_at_table() {
            println!("Seated at table!");
        }
        
        // 可以调用父模块的函数
        pub fn greet_guest() {
            println!("Greeting guest!");
            // 使用 super 关键字访问父模块
            super::seat_at_table();
        }
    }
    
    // 定义另一个子模块
    pub mod serving {
        pub fn take_order() {
            println!("Order taken!");
        }
        
        pub fn serve_food() {
            println!("Food served!");
        }
    }
}

// 定义另一个模块
mod back_of_house {
    // 定义结构体
    pub struct Chef {
        pub name: String,
        experience: u32,
    }
    
    // 为结构体实现方法
    impl Chef {
        pub fn new(name: String, experience: u32) -> Self {
            Chef {
                name,
                experience,
            }
        }
        
        pub fn cook_food(&self) {
            println!("{} is cooking food!", self.name);
        }
    }
    
    // 定义枚举
    pub enum Dish {
        Pizza,
        Pasta,
        Salad,
    }
    
    // 为枚举实现方法
    impl Dish {
        pub fn describe(&self) {
            match self {
                Dish::Pizza => println!("A delicious pizza!") ,
                Dish::Pasta => println!("A tasty pasta!") ,
                Dish::Salad => println!("A fresh salad!") ,
            }
        }
    }
}

// ===============================================================================
// 2. 路径（绝对路径和相对路径）
// ===============================================================================

fn main() {
    println!("=== 1. 模块定义和使用 ===");
    
    // 方法 1：使用绝对路径（从 crate 根开始）
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 方法 2：使用相对路径（从当前模块开始）
    front_of_house::hosting::greet_guest();
    
    // 方法 3：使用 use 声明简化路径
    use crate::front_of_house::serving;
    serving::take_order();
    serving::serve_food();

    println!("\n=== 2. 可见性（pub 关键字）===");
    
    // 创建 Chef 实例（Chef 是 pub 的，name 字段是 pub 的）
    let chef = back_of_house::Chef::new(String::from("Mario"), 10);
    println!("Chef name: {}", chef.name);
    chef.cook_food();
    // println!("Chef experience: {}", chef.experience); // 编译错误：experience 是私有的
    
    // 创建 Dish 实例
    let dish = back_of_house::Dish::Pizza;
    dish.describe();

    println!("\n=== 3. use 声明 ===");
    
    // 简化路径
    use crate::back_of_house::Chef;
    use crate::back_of_house::Dish;
    
    let chef2 = Chef::new(String::from("Luigi"), 5);
    println!("Chef name: {}", chef2.name);
    
    let dish2 = Dish::Pasta;
    dish2.describe();
    
    // 为类型创建别名
    use crate::back_of_house::Chef as HeadChef;
    let chef3 = HeadChef::new(String::from("Giovanni"), 15);
    println!("Head chef name: {}", chef3.name);

    println!("\n=== 4. 嵌套模块 ===");
    
    // 嵌套模块的路径
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    hosting::greet_guest();

    println!("\n=== 5. 模块的使用场景 ===");
    println!("1. 组织代码：将相关功能放在同一个模块中");
    println!("2. 控制可见性：使用 pub 关键字控制哪些功能对外可见");
    println!("3. 避免命名冲突：使用模块隔离不同的功能");
    println!("4. 复用代码：在不同的模块中复用相同的代码");
}

// ===============================================================================
// 6. 文件系统模块
// ===============================================================================

/*
Rust 支持将模块分散到不同的文件中：

项目结构：
src/
  main.rs
  front_of_house/
    mod.rs
    hosting.rs
    serving.rs

1. front_of_house/mod.rs：
   pub mod hosting;
   pub mod serving;

2. front_of_house/hosting.rs：
   pub fn add_to_waitlist() {
       println!("Added to waitlist!");
   }

3. front_of_house/serving.rs：
   pub fn take_order() {
       println!("Order taken!");
   }

4. main.rs：
   mod front_of_house;
   use crate::front_of_house::hosting;
   
   fn main() {
       hosting::add_to_waitlist();
   }
*/

// ===============================================================================
// 7. 模块的可见性规则
// ===============================================================================

/*
Rust 的可见性规则：
1. 模块内部的项默认是私有的
2. 使用 pub 关键字使项对外可见
3. pub 关键字可以用于模块、结构体、枚举、函数、方法、字段等
4. 对于结构体，pub 关键字使结构体对外可见，但字段默认是私有的
5. 对于枚举，pub 关键字使枚举对外可见，所有变体也对外可见
6. 使用 super 关键字访问父模块
7. 使用 self 关键字访问当前模块
*/

// ===============================================================================
// 8. 模块的最佳实践
// ===============================================================================

/*
模块的最佳实践：
1. 按照功能组织模块：将相关的功能放在同一个模块中
2. 保持模块的粒度适中：不要将过多的功能放在同一个模块中
3. 使用清晰的命名：模块名称应该清晰地反映其功能
4. 合理使用可见性：只将需要对外暴露的功能设为 pub
5. 使用 use 声明简化路径：避免重复书写长路径
6. 为类型创建别名：使用 as 关键字为长路径创建别名
7. 遵循文件系统结构：将模块分散到不同的文件中，提高代码的可读性
*/

// ===============================================================================
// 9. 模块与其他语言的对比
// ===============================================================================

/*
与其他语言的对比：
1. Java：使用包（package）组织代码，与 Rust 的模块类似
2. Python：使用模块（module）和包（package）组织代码，与 Rust 的模块类似
3. C++：使用命名空间（namespace）组织代码，与 Rust 的模块类似
4. Go：使用包（package）组织代码，与 Rust 的模块类似

Rust 模块系统的优势：
- 更灵活的可见性控制
- 更清晰的路径结构
- 更好的代码组织
- 更好的编译时检查
*/

// ===============================================================================
// 10. 模块的高级特性
// ===============================================================================

/*
模块的高级特性：
1. 条件编译：使用 #[cfg] 属性根据条件编译模块
2. 测试模块：使用 #[cfg(test)] 属性定义测试模块
3. 文档模块：使用 #[doc] 属性为模块添加文档
4. 私有模块：使用 pub(crate)、pub(super) 等控制可见性范围
*/

// 示例：私有模块
mod internal {
    // 只在当前 crate 中可见
    pub(crate) fn internal_function() {
        println!("Internal function!");
    }
    
    // 只在当前模块和父模块中可见
    pub(super) fn super_function() {
        println!("Super function!");
    }
}

// 调用私有模块的函数
fn call_internal_functions() {
    internal::internal_function();
    internal::super_function();
}