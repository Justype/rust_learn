# Struct

## 定义与实例化

### 定义

- 使用 `struct` 关键字并为整个结构体提供一个名字
- 结构体的名字需要描述它所组合的数据的意义
- 在大括号中，定义每一部分数据的名字和类型，即字段（field）

```rust
// 用户 struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,   // 最后一个可以有逗号
}
```

### 实例化

- 以结构体的名字开头
- 在大括号中使用 key: value 的形式提供字段
    - 顺序不需要和它们在结构体中声明的顺序一致
    - 字段必须要全部初始化

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### 获取与更改值

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

println!("UserName : \"{}\"", user1.email);

user1.email = String::from("anotheremail@example.com");
```

实例可变后，所有字段都是可变的。Rust 并不允许只将某个字段标记为可变。

### 函数返回 struct

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

### 初始化简写

变量名与字段名都完全相同

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### 更新语法：从其他实例创建实例

- `..实例`
- 使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例

不使用

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

更新语法

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### tuple struct

- `struct 名字(类型, 类型, 类型)`
- 整体有名，但元素没名。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

这里的 `black` 和 `origin` 是两种类型

### 类单元结构体 unit-like structs

- `{}`
- 没有任何字段的结构体
- 适用于：在某个类型上实现 trait 但不需要在类型中存储数据

### struct 所有权

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

- 这里的字段使用了 `String` 而不是 `&str`
    - 这个结构体拥有它所有的数据
    - 只要整个结构体是有效的，其数据也是有效的
- struct 里面也可以放引用，但需要使用生命周期
    - 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。

## 例子：计算长方形的面积

### 简单的函数

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

但是 `width`和`height` 是相关联的，不过程序本身却没有表现出这一点。

### 元组重构

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

缺点：可读性差，必须牢记 width 的元组索引是 0，height 的元组索引是 1

### 结构体重构

使用结构体为数据命名来为其赋予意义

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### 通过派生 trait 增加实用功能

#### 不能直接`println!`这个结构体

```rust
// `Rectangle` doesn't implement `std::fmt::Display`

println!("{}", rec);
//             ^^^ `Rectangle` cannot be formatted with the default formatter

```

提示信息：
- help: the trait `std::fmt::Display` is not implemented for `Rectangle`
- note: in format strings you may be able to use `{:?}` (or {:#?}for pretty-print) instead
- note: required by `std::fmt::Display::fmt`
- note: this error originates in a macro (in Nightly builds, runwith -Z macro-backtrace for more info

#### 修改为`{:?}`

```rust
// `Rectangle` doesn't implement `Debug`
println!("{:?}", rec);
//               ^^^ `Rectangle` cannot be formatted using `{:?}`
```

提示：
- help: the trait `Debug` is not implemented for `Rectangle`
- note: add `#[derive(Debug)]` or manually implement `Debug`
- note: required by `std::fmt::Debug::fmt`
- note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info

#### 给结构体添加：`#[derive(Debug)]`，增加注解来派生 `Debug` trait

```rust
fn main() {
    let rec = Rectangle { width: 30, height: 50 };

    println!("The area of this rectangle is {}.", area(&rec));

    println!("{:?}", rec);  // 使用调试格式打印 Rectangle 实例
}

#[derive(Debug)]    // 增加注解来派生 Debug trait
struct Rectangle {
    width : u32,
    height : u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

输出：
```
The area of this rectangle is 1500.
Rectangle { width: 30, height: 50 }
```

如果是`println!("{:#?}", rec);`，输出
```
Rectangle {
    width: 30,
    height: 50,
}
```

## struct 方法 method

- 与函数相似：`fn` 关键字、名称声明、参数、返回值、包含执行代码 
- 不同
    - 在结构体的上下文中被定义 （或者是枚举（6章）或 trait 对象的上下文（17章））
    - 第一个参数是`self`表示方法被调用的struct实例，（与python相似？）

### 定义 方法

- 在 `impl` 块里定义
    - implementation
    - 第一个参数 `self`
    - 方法可以选择获取 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。

```rust
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

#### 方法调用的运算符

C/C++ 两种运算符调用方法
- `.` 直接在对象上调用方法
    - `(*object).something()`
- `->` 在一个对象的指针上调用方法
    - `object->something()`

Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。

当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。

```rust
p1.distance(&p2);
(&p1).distance(&p2);
// 二者等价
```

#### 更多参数

看一个长方形是否能放下另一个

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // Can rect1 hold rect2? true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // Can rect1 hold rect3? false
}
```

### 关联函数

- 不把`self`作为第一个参数的函数（不是方法）
    - 例如`String::from`
- 通常用于构造器
- `::` 调用

例子：
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let squ1 = Rectangle::square(30);
}
```

### 多个 impl 块

每个结构体都允许拥有多个 impl 块。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```