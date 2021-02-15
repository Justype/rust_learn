# 枚举与模式匹配

## 枚举 enums: enumerations

### 定义

`enum 名称 { 成员1, 成员2, ... }`
- 成员（variants）

例子：可能的 IP 地址类型

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

### 枚举值

创建 IpAddrKind 两个不同成员的实例：

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- 枚举的成员位于其标识符的命名空间中，并使用两个冒号分开
- 优点：`IpAddrKind::V4` 和 `IpAddrKind::V6` 都是 `IpAddrKind` 类型
    - 可以定义一个函数来获取任何 `IpAddrKind`
    - 结构体存放一个 `IpAddrKind` 类型

parameter
```rust
fn main() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_type: IpAddrKind) { }
```

struct
```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

### 将任意类型的数据放入枚举成员中

- 字符串、数字类型或者结构体。甚至可以包含另一个枚举
- 优点
    - 不需要额外使用 struct
    - 每个变体可以拥有不同类型以及关联的数据量

例如

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}
```

再一个

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### 定义方法

与结构体一致

```rust
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## `Option<T>`

- Option 是标准库定义的一个枚举
- 类似`Null`概念的枚举

Rust 没有 `Null`
- 问题：当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误
- 但，空值的概念仍是有意义的：空值是一个因为某种原因目前无效或缺失的值

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### 使用

实例化？

```rust
let some_number = Some(5);          // 能自动推断类型
let some_string = Some("a string");

let absent_number: Option<i32> = None;  // 不能自动推断类型
```

优点：`Option<T>` 与 `T` 是不同类型，不能直接使用

```rust
// the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not satisfied
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
//          ^ no implementation for `i8 + std::option::Option<i8>`
```

使用`Option<T>`时，必须明确的处理值为空的情况

## match 控制流运算符

感觉相当于C#的`value switch`

其匹配模式可由字面值、变量、通配符和许多其他内容构成

例子：硬币=>美分值

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

多行代码
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 绑定值的模式

- 绑定匹配的模式的部分值
    - 从枚举成员中提取值

```rust
#[derive(Debug)] // 便于调试的打印
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {   // 绑定到了到了 UsState
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

### 匹配 `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 必须穷举所有可能

```rust
// non-exhaustive patterns: `None` not covered
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
    //    ^ pattern `None` not covered
        Some(i) => Some(i + 1),
    }
}
```

#### _ 通配符

相当于其他语言的`default`？ 其实就是不关心那个值，把它抛弃

如果我们只关心 1、3、5 和 7 这几个值。

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

## if let

只关心一种匹配，忽略其他的。

### 例子：只关心 3

match
```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

if let
```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

### 与match比较
- 使用 if let 意味着编写更少代码
- 但会失去 match 强制要求的穷尽性检查

### 搭配 else
```rust
if let Some(3) = some_u8_value {
    println!("three");
} else {
    println!("others");
}
```
