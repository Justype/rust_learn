# Slice 类型

slice 允许你**引用**集合中一段连续的元素序列，而不用引用整个集合。

## 例子

编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

### 返回单词结尾的索引

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();   // 将 String 转化为字节数组

    for (i, &item) in bytes.iter().enumerate() {
        // 使用 iter 方法在字节数组上创建一个迭代器
        // enumerate 方法返回一个元组，使用模式匹配来解构
        // i 是索引，而 &item 是单个字节
        if item == b' ' {
            return i;   // 如果找到了一个空格，返回它的位置
        }
    }

    s.len() // 如果没找到，返回字符串的长度
}
```

局限：返回了一个独立的 usize，不过它只在 &String 的上下文中才是一个有意义的数字。换句话说，因为它是一个与 String 相分离的值，无法保证将来它仍然有效。

例子：

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}
```

### 字符串切片

字符串 slice（string slice）是 String 中一部分值的引用

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // [] 里面放入 range
let world = &s[6..11];
```

![slice](assets/04-06.svg)

语法糖 | 代码
-- | --
从头开始 | `[..5]`
到尾结束 | `[6..]`
从头到尾 | `[..]`

    注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。

#### 重写例子

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

调用

```rust
// cannot borrow `s` as mutable because it is also borrowed as immutable
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    //                    -- immutable borrow occurs here

    s.clear(); // 错误!
//  ^^^^^^^^^ mutable borrow occurs here

    println!("the first word is: {}", word);
    //                                ---- immutable borrow later used here
}
```

回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 clear 需要清空 String，它尝试获取一个可变引用。Rust不允许这样做，因而编译失败。Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！

## 字符串字面值就是 slice

```rust
let s = "Hello Rust!";
```

这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

### 字符串 slice 作为参数

在知道了能够获取字面值和 String 的 slice 后，我们对 first_word 做了改进，这是它的签名：

```rust
fn first_word(s: &String) -> &str {
```

而更有经验的 Rustacean 会编写出示例 4-9 中的签名，因为它使得可以对 String 值和 &str 值使用相同的函数：

```rust
fn first_word(s: &str) -> &str {
```

如果有一个字符串 slice，可以直接传递它。如果有一个 String，则可以传递整个 String 的 slice。定义一个获取字符串 slice 而不是 String 引用的函数使得我们的 API 更加通用并且不会丢失任何功能：

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 就是 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
}
```

## 其他类型的 slice

数组

```rust
let a = [1, 2, 3, 4, 5];

let slice: &[i32] = &a[1..3]; // 注意类型
```