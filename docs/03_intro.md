# 热身：猜数游戏

## version 0.1 输入输出

- 基本的库引用
    - `using std::io`
- 变量声明
    - `let aaa = bbb`
- `mut` 关键字
    - 一般的变量与引用是不可变的
    - 可变的变量与引用 要加`mut`
- 输入输出
    - input : `std::io::stdin()`
    - output : `println!()`

```rust
use std::io;    // 引用标准库下的 io库

// Rust 会默认导入 prelude 模块

fn main() {
    println!("这是一个猜数游戏");
    println!("请猜测一个数：");

    // 在 Rust 下，一般声明变量是不可变的
    let mut guess = String::new();
    // :: 调用的是关联函数，相当于 静态方法

    io::stdin().read_line(&mut guess)   // 传入的是 变量的引用
        // 引用在 Rust 内也是不可变的，所以也要加 mut
        .expect("无法读取行");  // 如果出现错误，调用 expect 函数
    
    // read_line() 会返回两种枚举类型
    // io::Result  Ok, Err
    // .expect 是 io.Result 定义的方法   如果结果为 Err，会中断程序，打印字符串


    println!("你猜测的数字是：{}", guess);
    // {} 为占位符，会将后面的参数以此填入
}
```

## version 0.2 生成随机数

- [crates.io](https://crates.io)
    - 用于查找 `Rust` 库的网站
- 安装 `crate`
    - 在 `Cargo.toml` 下的 `[dependencies]`下 添加
    - 使用方法：`包名 = "版本"`
        - `rand = "0.3.14"`
        - `rand = "^0.3.14"`
            - 任何与`0.3.14`版本共用API 所兼容的版本
    - 修改 `Cargo.toml` 后，再运行 `cargo build`
        - 会自动下载相关依赖
- 包版本控制      ？？有点不懂
    - 为了保证程序的一致性
        - 在执行一次`cargo build`后，相关的版本会被写到`Cargo.lock`文件下
        - 再次执行`cargo build`，会先查找`Cargo.lock`文件 下的版本
    - 执行 `cargo update` 后，就会更新到最新的小版本
        - 原来是 `0.3.14` 目前最新版为 `0.8.3` 小版本最新为 `0.3.25`
        - 执行后 更新到 `0.3.23`，写入到 `Cargo.lock` 文件，但不改变 `Cargo.toml`
    - 想要升级到大版本最新，要更改 `Cargo.toml` 文件

``` rust
use std::io;
use rand::Rng;  // 引入 rand 库


fn main() {
    println!("这是一个猜数游戏");
    println!("请猜测一个数：");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..101);
    // 注意：0.8.0 后 的 gen_range() 传入的是 range 类型，而不是两个数字了

    println!("生成的数字是{}", secret_number);

    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测的数字是：{}", guess);
}
```

## version 0.3 条件判断

1. `Rust` 可通过 `变量.cmp(变量2)` 进行比较
    - 返回 `std::cmp::Ordering` 的枚举类型
        - `Ordering::Less`, `Ordering::Greater`, `Ordering::Equal`
2. 显式声明变量类型：`let guess: u32`
3. `Rust` 有变量类型推断的功能
    - 本来 `secret_number` 是 `i32`
    - 但 有后来的语句：`match guess.cmp(&secret_number)`
        - `guess` 被指定为 `u32`
        - 只能同类型比较
    - `secret_number` 就被推断为 `u32`

```rust
use rand::Rng;
use std::cmp::Ordering;  // 比较的结果，是一种枚举类型   cmp compare
use std::io; // 引用标准库下的 io库


fn main() {
    println!("这是一个猜数游戏");
    println!("请猜测一个数：");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("生成的数字是{}", secret_number);

    io::stdin().read_line(&mut guess).expect("无法读取行");

    // 转换数字
    let guess: u32 = guess.trim().parse().expect("请输入数字"); 
    // shadow 把之前的变量给隐藏了
    // 方便了变量类型转换的问题  复用了变量名，而不是 guessString, guessI32

    println!("你猜测的数字是：{}", guess);

    // 比较数字
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("小了"), // 注意这里是 ::
        Ordering::Greater => println!("大了"),
        Ordering::Equal => println!("你赢了"),
    }
    // 感觉 match 与 switch 语句相似，通过匹配，选择走向
}
```

## version 1.0 循环

- 使用 `loop` 无限循环
    - `break` 退出
    - `continue` 继续下一个循环
- 使用 `_` 放弃结果

```rust
use rand::Rng;
use std::cmp::Ordering; // 比较的结果   cmp compare
use std::io; // 引用标准库下的 io库

// Rust 会默认导入 prelude 模块

fn main() {
    println!("！！猜数游戏！！");


    let secret_number = rand::thread_rng().gen_range(1..101); // 生成随机数

    println!("生成的数字是{}", secret_number);

    loop {  // 无限循环
        println!("请猜测一个数：");

        let mut guess = String::new(); // 新建一个字符串
        // 如果把 guess 放到循环外，就会比较失败？   只能比较第一次，后边就没有结果了
        // 懂了，这也是一次 shadow 因为第二次循环开始时， guess 是 u32 类型
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // 转换数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // _ 忽略，不要该结果了，与C#相似
                println!("请输入数字");
                continue;
            }
        }; // 这也与 C# 的 value switch 比较相似

        println!("你猜测的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            // 比较数字
            Ordering::Less => println!("小了"), // 注意这里是 ::
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
        }
        // 感觉 match 与 switch 语句相似，通过匹配，选择走向
    }
}
```
