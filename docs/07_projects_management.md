# 项目管理

Rust 代码组织
- 哪些暴露，哪些私有
- 作用域内哪些名称是有效的

从大到小
- 包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
- Crates ：一个模块的树形结构，它形成了库或二进制项目。
- 模块（Modules）和 use： 允许你控制作用域和路径的私有性。
- 路径（path）：一个命名例如结构体、函数或模块等项的方式

## Package, Crate

crate 类型：
- binary 二进制
- library 库

crate Root：
- 源代码文件
- Rust 编译器以它为起始点，并构成 crate 的根模块

**一个Package：**
- 包含一个`Cargo.toml`（描述如何构建这些Crates）
- 只能包含一个 library crate
- 可包含任意数量的 binary crate
- 但至少包含一个crate

重新观察 cargo new：
```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

创建了一个 binary package

### 惯例

- `src/main.rs`
    - binary crate 的 crate root
    - crate名 与 package名 相同
- `src/lib.rs`
    - package 包含一个 library crate
    - library crate 的 crate root
    - crate名 与 package名 相同
- Cargo 把 crate root 文件交给 rustc 来构建 library 或 binary

- 一个Package可以同时包含`src/main.rs`和`src/lib.rs`
    - 一个binary crate，一个library crate
    - 名称与package名相同
-   一个Package可以有多个binary crate：
    - 文件放在src/bin
    - 每个文件是单独的binary crate

### Crate 作用

- 将相关功能组合到一个作用域内，便于项目间进行共享
    - 防止冲突
- 例：访问rand crate 的功能需要通过它的名字：`rand`

## Module 控制作用域和巳酉星

- Module：
    - 在一个crate内，将代码进行分组
    - 增加可读性，易于复用
    - 控制条目（item）的私有性：public, private
- 建立module：
    - `mod` 关键字
    - 可嵌套
    - 可包含其它项(struct、常量、trait、函数等）的定义

### 例子：饭店前台

lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}
```

模块树（module tree）
```
crate (lib.rs)
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

siblings 同, child 子, parent 父

## path

- 绝对路径（absolute path）
    - 从 crate 根开始，
    - 以 crate 名或者字面值 crate 开头。
- 相对路径（relative path）
    - 从当前模块开始，
    - 以 self、super 或当前模块的标识符开头。

路径至少有一个标识符，标识符之间用`::`

### 例子

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径 Absolute path
    // crate 指的是这个文件 lib.rs
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径 Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

- 如果定义和使用都在一起，可使用相对路径
- 如果定义与使用分开，使用绝对路径
- 建议使用绝对路径

### 私有边界 privacy boundary

- 模块不仅可以组织代码，还可以定义私有边界。
- 如果想把函数或struct等设为私有，可以将它放到某个模块中
- 所有项（函数、方法、结构体、枚举、模块和常量）都是**默认私有**的
- 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项

所以上面的会报错

```rust
// error[E0603]: module `hosting` is private
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    //                     ^^^^^^^ private module

    front_of_house::hosting::add_to_waitlist();
    //              ^^^^^^^ private module
}
```

### super

- 相对路径的父级
- 相当于文件系统的`..`

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();   // 子级可以调用父级的函数

        crate::serve_order();   // 绝对路径
    }

    fn cook_order() {}
}
```

### pub

让 私有的所有项暴露

一：暴露模块，仍报错
```rust
// error[E0603]: function `add_to_waitlist` is private
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    //                              ^^^^^^^^^^^^^^^ private function
}
```

二：暴露方法
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
```

sibling 之间可以相互调用

#### pub struct

pub struct
- struct 是公共的
- struct 的字段默认是私有的
    - 什么字段想公有，加pub

例子
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // 构造 Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye（黑麦） toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    // 现在要小麦面包了
    meal.toast = String::from("Wheat"); 
    println!("I'd like {} toast please", meal.toast);

    // 私有字段无法访问
    meal.seasonal_fruit = String::from("blueberries");
}
```

#### pub enum

`pub enum` 所有变体也都公共

```rust
mod back_of_house {
    pub enum Appetizer { // 开胃菜
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## use

使用 use 关键字将名称引入作用域

`src/lib.rs`
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 将 hosting 模块引入作用域
use crate::front_of_house::hosting; // 绝对路径
// use front_of_house::hosting; // 相对路径

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

惯例
- 函数：建议**引入父级**，不直接引入
    - 因为代码多了，难以判断该函数是本地定义的还是引用的
- struct, enum：直接引入
- 同名条目，指定到父级
    - `fmt::Result` 与 `io::Result`

### use as

- 指定本地别名
- 相当于 python 的 import as

### pub use

- 当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。
- 为了让外部访问，要用`pub use` 重导出（re-exporting）

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- 这样暴露的结构，可能与定义的结构不同。
- 有用：代码的内部结构与调用代码的程序员的思考领域不同时

### 使用外部包

1. `Cargo.toml` 添加依赖的包
2. `use` 引入

crates.io 镜像源修改：`~/.cargo/config`

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 你想换的源
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

#### 嵌套路径

`相同的部分::{不同的部分}`

```rust
use std::cmp::Ordering;
use std::io;
// 相当于
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// 相当于
use std::io::{self, Write};
```

#### 通配符 *

引入所用公共条目

使用场景：
- 测试：将所有被测试代码引入到tests模块
- 有时被用于预导入(prelude)模块

## 将模块内容移动到其他文件

- 模块定义时，如果模块名后边是`;`，而不是代码块：
    - Rust会从与模块同名的文件中加载内容
    - 模块树的结构不会变化
- 文件的层级结构，必须与模块的层级结构相同

### 例子

#### 原来

`src/lib.rs`
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

#### 将`front_of_house`放到单独的文件内

`front_of_house.rs`
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

`lib.rs`
```rust
mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

#### 将`hosting`放到单独的文件

文件的层级结构，必须与模块的层级结构相同

`front_of_house/hosting.rs`
```rust
pub fn add_to_waitlist() {}
```

`front_of_house.rs`
```rust
pub mod hosting;
```