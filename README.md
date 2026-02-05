# Rust 学习指南

本项目旨在为 Rust 初学者提供一套系统的学习资源，通过示例代码展示 Rust 的核心概念和最佳实践。

## 一、Rust 语言简介

### 语言特性
- **内存安全**：通过所有权（Ownership）、借用（Borrowing）和生命周期（Lifetimes）系统，在编译时保证内存安全，无需垃圾回收器。
- **高性能**：接近 C/C++ 的运行速度，支持零成本抽象。
- **并发性**：通过类型系统和所有权规则，在编译时防止数据竞争。
- **可靠性**：强大的类型系统和错误处理机制，减少运行时错误。
- **现代化语法**：模式匹配、 trait 系统、泛型等现代语言特性。

### 历史
- 2006 年：Mozilla 员工 Graydon Hoare 开始设计 Rust。
- 2010 年：Mozilla 宣布赞助 Rust 项目。
- 2015 年：Rust 1.0 稳定版发布。
- 2018 年至今：Rust 每年发布一个主要版本，持续改进语言特性和工具链。
- 连续多年被 Stack Overflow 开发者调查评为「最受喜爱的编程语言」。

### 应用领域
- **系统编程**：操作系统、驱动程序、嵌入式系统。
- **网络服务**：高性能服务器、API 后端。
- **区块链**：如 Solana、Polkadot 等项目使用 Rust 开发。
- **游戏开发**：如 Amethyst 游戏引擎。
- **Web 开发**：通过 WebAssembly 技术，Rust 可以在浏览器中运行。
- **工具开发**：如 npm、Discord、Figma 等公司使用 Rust 开发工具。

## 二、Rust 环境配置

### 使用 rustup 安装 Rust

`rustup` 是 Rust 的官方安装工具，用于管理 Rust 版本和相关工具链。

#### 安装步骤

1. **在 Linux 或 macOS 上安装**：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **在 Windows 上安装**：
   - 访问 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
   - 下载并运行 rustup-init.exe 安装程序

3. **验证安装**：
   ```bash
   rustc --version
   cargo --version
   ```

#### 常用 rustup 命令

- **更新 Rust**：
  ```bash
  rustup update
  ```

- **查看已安装的工具链**：
  ```bash
  rustup show
  ```

- **安装特定版本**：
  ```bash
  rustup install nightly
  ```

- **切换默认工具链**：
  ```bash
  rustup default nightly
  ```

## 三、代码示例索引

本项目 `src/bin` 目录包含以下代码示例，按 Rust 学习的逻辑顺序排列：

| 文件名 | 主题 | 内容简介 |
|-------|------|----------|
| [01_variables.rs](src/bin/01_variables.rs) | 变量 | 变量声明、可变性、常量、遮蔽 |
| [02_data_types.rs](src/bin/02_data_types.rs) | 数据类型 | 标量类型、复合类型 |
| [03_functions.rs](src/bin/03_functions.rs) | 函数 | 函数定义、参数、返回值 |
| [04_control_flow.rs](src/bin/04_control_flow.rs) | 控制流 | if 表达式、循环（loop、while、for） |
| [05_ownership.rs](src/bin/05_ownership.rs) | 所有权 | 所有权规则、移动语义、借用、切片 |
| [06_structs.rs](src/bin/06_structs.rs) | 结构体 | 结构体定义、方法、关联函数 |
| [07_enums.rs](src/bin/07_enums.rs) | 枚举 | 枚举定义、Option 类型、match 表达式 |
| [08_collections.rs](src/bin/08_collections.rs) | 集合 | Vector、String、HashMap |
| [09_modules.rs](src/bin/09_modules.rs) | 模块 | 模块系统、路径、pub 关键字 |
| [10_error_handling.rs](src/bin/10_error_handling.rs) | 错误处理 | Result 类型、panic!、错误传播 |
| [11_generics_traits.rs](src/bin/11_generics_traits.rs) | 泛型和特质 | 泛型、trait 定义和实现 |
| [12_lifetimes.rs](src/bin/12_lifetimes.rs) | 生命周期 | 生命周期注解、借用检查器 |
| [13_macro.rs](src/bin/13_macro.rs) | 宏 | 声明式宏、过程宏 |

### 运行示例代码

要运行单个示例文件，使用以下命令：

```bash
# 编译并运行特定文件
rustc src/bin/01_variables.rs -o variables && ./variables

# 或使用 cargo run（如果配置了 Cargo.toml）
cargo run --bin 01_variables
```

### 学习路径建议

1. **基础概念**：变量 → 数据类型 → 函数 → 控制流
2. **核心特性**：所有权 → 结构体 → 枚举 → 集合
3. **模块化与抽象**：模块 → 泛型和特质 → 生命周期
4. **高级特性**：错误处理 → 宏

## 四、学习资源

- **官方文档**：[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)
- **练习网站**：[Rustlings](https://github.com/rust-lang/rustlings)
- **在线编译器**：[Rust Playground](https://play.rust-lang.org/)
- **社区论坛**：[Rust 中文社区](https://rustcc.cn/)

## 五、贡献指南

欢迎提交 Issue 和 Pull Request 来改进本项目！

## 六、许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件
