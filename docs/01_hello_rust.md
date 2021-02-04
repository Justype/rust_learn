# 编写 Rust 程序

- 程序文件后缀名：`.rs`
- 命名规范：小写，用下划线分隔 `hello_world.rs`

## 编译与运行 Rust 程序

- 编译：`rustc main.rs`
- 运行：
    - Windows:`.\main.exe`
    - Linux/Unix:`./main`

## 解析 Rust 程序

```rust
// 定义函数
fn main() { // main 函数为入口
    println!("Hello Rust!"); // 这个是宏，不是函数
    // "Hello Rust" 为字符串
}
```

- 定义函数 `fn`
- 注意
    - 缩进为四个空格
    - 代码以`;`结尾
    - 带`!`为宏(Rust macro)，而不是函数

## 编译和运行是单独的两步

- 运行Rust程序之前必须先编译，命令为：`rustc 源文件名`
- 编译成功后，会生成一个二进制文件
    - 在Windows上还会生成一个`.pdb`文件，里面包含调试信息
- Rust是AOT编译的语 (ahead-of-time)
    - 可以先编译程序，然后把可执行文件交给别人运行（无需安装Rust)
- rustc只适合简单的Rust程序
- 复杂的程序需要`cargo`

