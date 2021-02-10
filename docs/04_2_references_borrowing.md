# 引用与借用

 `&` 符号就是引用，它们允许你使用值但不获取其所有权

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s 是对 String 的引用
    s.len()
}// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权,所以什么也不会发生
```

![References](assets/04-05.svg)

`&s1` 语法让我们创建一个 指向 值 `s1` 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。

**借用**：获取引用作为函数参数

## 引用 默认不可变

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

// cannot borrow immutable borrowed content `*some_string` as mutable
fn change(some_string: &String) {
//                     ------- use `&mut String` here to make mutable
    some_string.push_str(", world");
//  ^^^^^^^^^^^ cannot borrow as mutable
}
```

## 可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 限制：一个块只能有一个引用

限制：在特定作用域中的特定数据有且只有一个可变引用。

```rust
// cannot borrow `s` as mutable more than once at a time
let mut s = String::from("hello");

let r1 = &mut s;
//       ------ first mutable borrow occurs here
let r2 = &mut s;
//       ^^^^^^ second mutable borrow occurs here

println!("{}, {}", r1, r2);
//                 -- first borrow later used here
```

### 数据竞争 data race

- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据访问的机制。

数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能**同时**拥有

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

let r2 = &mut s;
```

### 不能同时存在可变引用和不可变引用

但可同时存在多个不可变引用

```rust
// cannot borrow `s` as mutable because it is also borrowed as immutable
let mut s = String::from("hello");

let r1 = &s; // 没问题
//       -- immutable borrow occurs here
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题
//       ^^^^^^ mutable borrow occurs here

println!("{}, {}, and {}", r1, r2, r3);
//                         -- immutable borrow later used here
```

### 悬垂引用 Dangling References

悬垂指针：其指向的内存 可能已经被释放并分配给其它持有者

```rust
// missing lifetime specifier
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    //         ^ expected lifetime parameter
    let s = String::from("hello");

    &s
}   // 这里 s 离开作用域并被丢弃。其内存被释放。
```

解决方法是直接返回 String：

```rust
// missing lifetime specifier
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    s   // 所有权被移动出去，所以没有值被释放。
}
```

## 引用规则

- 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
- 引用必须总是有效的。