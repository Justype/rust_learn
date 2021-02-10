# 所有权 Ownership

- 所有权是Rust最独特的特性，它让Rust无需GC就可以保证内存安全。

## 什么是所有权

- Rust的核心特性就是所有权
- 所有程序在运行时都必须管理它们使用计算机内存的方式
    - 有些语言（C#、Java）有垃圾收集机制，在程序运行时，它们会不断地寻找不再使用的内存
    - 在其他语言（C、C++）中，程序员必须显式地分配和释放内存
- Rust采用了第三种方式：
    - 内存是通过一个所有权系统来管理的，其中包含一组编译器在编译时检查的规则。
    - 当程序运行时，所有权特性不会减慢程序的运行速度。
        - 把内存管理的相关工作提到了编译时

## Stack vs Heap

栈内存 vs 堆内存
- 在像Rust这样的系统级编程语言里，一个值是在stack上还是在heap上对语言的行为和你为什么要做某些决定是有更大的影响的
- 在你的代码运行的时候，Stack和Heap都是你可用的内存，但他们的结构很不相同。

### 存储数据

- Stack 栈：后进先出
    - 后进先出 LIFO (Last in First off)
        - 存入数据叫 压栈
        - 移除数据叫 出栈
    - 存储在栈上的数据必须拥有已知的固定的大小
- Heap 堆
    - 内存组织性差一点
        - 当你把数据放入heap时，你会请求一定数量的空间
        - 操作系统在heap里找到一块足够大的空间，把它标记为在用，并返回一个指针，也就是这个空间的地址
        - 这个过程叫“分配”，在heap上进行分配
    - 编译时大小未知的数据或运行时大小可能发生变化的数据必须存放在heap上

#### 指针
- 因为指针是已知的固定大小的，所以可以把指针存放在stack内
- 但如果想要实际数据，必须使用指针来定位。

#### 操作速度：压栈更快
- Stack：因为操作系统不需要寻找用来存储新数据的空间，那个位置永远都在stack的顶端
- Heap：操作系统首先需要找到一个足够大的空间来存放数据，然后要做好记录方便下次分配

#### 访问数据：栈更快
- 访问heap中的数据要比访问stack中的数据慢，因为需要通过指针才能找到heap中的数据
    - 对于现代的处理器来说，由于缓存的缘故，如果指令在内存中跳转的次数越少，那么速度就越快
- 如果数据存放的距离比较近，那么处理器的处理速度就会更快一些(stack上）
- 如果数据之间的距离比较远，那么处理速度就会慢一些(heap上）
    - 在heap上分配大量的空间也需要时间

#### 函数调用
当你的代码调用函数时，值被传入到函数（也包括指向heap的指针）。函数本地的变量被压到stack上。当函数结束后，这些值会从stack上弹出

### 所有权存在的原因

- 所有权解决的问题：
    - 跟踪代码的哪些部分正在使用heap的哪些数据
    - 最小化heap上的重复数据量
    - 清理heap上未使用的数据以避免空间不足。
    - 旦你懂的了所有权，那么就不需要经常去想stack或heap了。
- 但是知道管理heap数据是所有权存在的原因，这有助于解释它为什么会这样工作。



## 所有权规则

1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
2. 值有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

### 变量作用域 Scope

- Scope 就是程序中一个项目的有效范围

```rust
{                      // s 在这里无效, 它尚未声明
    let s = "hello";   // 从此处起，s 是有效的
    // 可以操作 s

}   // 此作用域已结束，s 不再有效
```

- 当 `s`进入作用域时，它就是有效的。
- 这一直持续到它**离开作用域**为止。

### String 类型

字符串字面值：
- 字符串值被硬编码进程序里
- 不可变的

String
- 存储在堆上
- 能够存储在编译时未知大小的文本

可以使用 `from` 函数基于字符串字面值来创建 String，如下：

```rust
let mut s = String::from("hello");

s.push_str(", rust!"); // push_str() 在字符串后追加字面值

println!("{}", s); // 将打印 `hello, rust!`
```

为什么 String 可变而字面值却不行呢？区别在于两个类型对内存的处理上。

### 内存与分配

字符串字面值：string literal
- 在编译时就知道其内容，
- 文本被直接硬编码进最终的可执行文件中

String 为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。
- 必须在运行时向操作系统请求内存。
    - `String::from()`
- 需要一个当我们处理完 String 时将内存返回给操作系统的方法。
    - 有GC，GC会记录并清除不再使用的内存
    - 无GC，必须手动释放内存
        - 忘记回收，浪费内存
        - 过早回收，出现无效变量
        - 重复回收，bug
    - Rust：内存在拥有它的变量离开作用域后就被自动释放。

```rust
{
    let s = String::from("hello"); // 从此处起，s 是有效的

    // 使用 s
}   // 此作用域已结束，s 不再有效
```

当变量离开作用域，Rust 会调用 `drop` 函数，释放内存

### 变量与数据交互方式

- 栈：拷贝
- 堆：移动，克隆

#### 移动 Move

整形的例子

```rust
let x = 5;  // 将 5 绑定到 x
let y = x;  // 接着生成一个值 x 的拷贝并绑定到 y

// x 和 y，都等于 5
// 因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中
```

```rust
let s1 = String::from("hello");
let s2 = s1;
```

String 由三部分组成
- 指向存放字符串内容内存的指针
- 长度：存放字符串内容所需的字节数
- 容量：String从操作系统总共获得的内存的总字节数

![s1](assets/05-01.svg)

将 s1 赋值给 s2，String 的数据被复制了
- 从栈上拷贝了它的指针、长度和容量。
- 但并没有复制指针指向的堆上数据。

![s2](assets/05-02.svg)

当变量离开作用域，Rust会自动调用`drop`，将使用的heap内存释放

所以当`s1`, `s2`离开作用域，它们都会尝试释放相同的内存
- 如果地址被多次释放会造成：二次释放（double free）的错误
    - 两次释放（相同）内存会导致内存污染，可能损坏正在使用的内存

为了确保内存安全，Rust 认为 `s1` 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西

![移动](assets/05-03.svg)

```rust
let s1 = String::from("hello");
//  -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
let s2 = s1;
//  -- value moved here

println!("{}", s1);
//             ^^ value borrowed here after move
```

浅拷贝（shallow copy）和 深拷贝（deep copy）

Rust 在这里的操作与浅拷贝不同，多了一步：让`s1`失效，所以被称为“移动”

设计原则：Rust不会自动创建数据的深拷贝

#### 克隆 Clone

深拷贝

![Clone](assets/05-04.svg)

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

#### 只在栈上的数据：拷贝 Copy

```rust
let x = 5;
let y = x;  // copy

println!("x = {}, y = {}", x, y);
```

trait 可以简单理解为接口，后面会详细了解

- 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用
- Rust 不允许自身或其任何部分实现了`Drop` trait 的类型使用`Copy` trait

那些类型实现了 Copy trait
- 任何简单标量值的组合
- 不需要分配内存或某种形式资源的类型

### 所有权与函数

- 将值传递给函数在语义上与给变量赋值相似。
- 向函数传递值可能会移动或者复制

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 为 i32 类型，Copy
                                    // 后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。
  // 但因为 s 的值已被移走，所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

### 返回值与作用域

函数在返回值的过程中也会发生所有权转移

```rust
fn main() {
    let s1 = gives_ownership();  // gives_ownership 将返回值移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。
  // s2 也移出作用域，但已被移走，所以什么也不会发生
  // s1 移出作用域并被丢弃

fn gives_ownership() -> String { // gives_ownership 将返回值移动给调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string  // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
```

变量的所有权总是遵循相同的模式：
- 将值赋给另一个变量时移动它。
- 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

如果没有引用，实现不给予所有权
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // s1 的所有权给了 s2

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
```