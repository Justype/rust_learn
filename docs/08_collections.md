# 集合

- Vector
- String
- HashMap

## Vector

- `Vec<T>`，叫做vector
    - 由标准库提供
    - 可存储多个值
    - 只能存储相同类型的数据
    - 值在内存中连续存放

### 创建

`Vec.new`

```rust
let v: Vec<i32> = Vec::new();
```

使用初始值创建，`vec!`

```rust
let v = vec![1, 2, 3];
```

### 增

`.push`

```rust
let mut v = Vec::new(); // 后面 push 了 i32 类型的值，所以推断出了类型，不用手动指明

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### 删除 vector

- vector 在其离开作用域时会被释放
- 当 vector 被丢弃时，所有其内容也会被丢弃
    - 当 vector 中的元素被引用的时候，就会变得复杂

### 读取

1. 索引`[]`
2. `.get()`

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];    // 在这里使用了引用
println!("第三个元素是 {}", third);

match v.get(2) {
    Some(third) => println!("第三个元素是 {}", third),
    None => println!("没有第三个元素"),
}
```

- 如果超出索引后想让程序崩溃：使用`[]`
- 反之，使用`.get()`

### 所有权与借用

同样适用，回顾一下：
- 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
- 引用必须总是有效的。

例子
```rust
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];
//           - immutable borrow occurs here

v.push(6);
//^^^^^^^ mutable borrow occurs here

println!("The first element is: {}", first);
//                                   ----- immutable borrow later used here
```

为什么第一个元素的引用会关心 vector 结尾的变化？
- 在 vector 的结尾增加新元素时，
    - 若没有足够空间将所有所有元素依次相邻存放
    - 会要求分配新内存并将老的元素拷贝到新的空间中
- 这时，第一个元素的引用就指向了被释放的内存
- 借用规则阻止程序陷入这种状况。

    关于 `Vec<T>` 类型的更多实现细节，在 https://doc.rust-lang.org/stable/nomicon/vec.html 查看 “The Nomicon”

### 遍历 for in

不可变
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

可变
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

### 例子 vector + enum (+ match)

利用 enum 存放多种类型的数据

```rust
enum SpreadsheetCell {  // 表格的单元格
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("蓝色")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

### 为什么需要知道 `vector<T>` 的 T

1. 要知道储存每个元素到底需要多少内存
2. 可以准确的知道这个 vector 中允许什么类型
    - 不同类型间的相同操作可能出错

## String

- Rust 倾向于确保暴露出可能的错误
- 字符串是比很多程序员所想象的要更为复杂的数据结构
- UTF-8

### 啥是字符串？

Rust 的**核心语言**中只有一种字符串类型：str，字符串切片，它通常以被借用的形式出现，&str。（它们是一些储存在别处的 UTF-8 编码字符串数据的引用。）

String
- 为标准库提供
- 可增长、可修改、可拥有
- UTF-8

谈到 Rust 的 “字符串”时，它们通常指的是 String 和字符串 slice &str 类型，而不仅仅是其中之一。

Rust 标准库中还包含一系列其他字符串类型，比如 OsString、OsStr、CString 和 CStr。相关库 crate 甚至会提供更多储存字符串数据的选择。

### 创建

很多`Vec<T>`的操作都可用于`String`

- `String::new()`函数
- `to_string()`方法 （实现了 Display trait的类型，含字符串字面值）
- `String::from()`函数

### 更新

- 添加
    - `push_str()`方法，添加`&str`，不获取所有权
    - `push()`方法，增加字符
- 拼接
    - `+`：`String + &str`
        - `+` 签名 `fn add(self, s: &str) -> String {`
            - String 会被移动，无法继续使用
        - 为啥明明是`&str`却能传入 &String
            - Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术
            - &String 可以被 强转（coerced）成 &str
    - format!("{}{}", s1, s2)
        - 返回`String`
        - 并且不会获取任何参数的所有权


例子
```rust
fn main() {
    let mut g1 = "\nHello".to_string();

    g1.push_str(" Rust");
    g1.push('!');

    let g2 = "\nHello World!".to_string();

    let g3 = g1 + &g2;

    // println!("greeting : {}", g1);  // 无法继续使用
    println!("greeting : {}", g2); // 可以
    println!("greeting : {}", g3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // s1 所有权没有丢失
    println!("{}", s);

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
}
```

### 无法索引字符串

```rust
// error[E0277]: the trait bound `std::string::String: std::ops::Index<{integer}>` is not satisfied
let s1 = String::from("hello");
let h = s1[0];
//      ^^^^^ the type `std::string::String` cannot be indexed by `{integer}`
```

因为UTF-8编码的字符，一个字符可能是1-2个字节。
```rust
String::from("Hola").len(); // 4
String::from("Здравствуйте").len(); // 24 not 12
```

`З`在内存中有两个字节：`[208, 151]` 使用`s[0]`获取到的是`208`，无实际意义

### 字节、标量值和字形簇

Rust 三种看待字符的方式
- 字节 Bytes
- 标量值 Scalar Values
- 字形簇 Grapheme Clusters （最接近“字母”）

नमस्ते （梵文书写的印度语）
- 字节：`[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]`
- Unicode 标量值：`['न', 'म', 'स', '्', 'त', 'े']`
- 字形簇：`["न", "म", "स्", "ते"]`
    - 获取字形簇是很复杂的，标准库并没有提供这个功能

### Rust不允许对String进行索引的最后一个原因：

- 索引操作应消耗一个常量时间(O(1))
- 而String无法保证：需要遍历所有内容，来确定有多少个合法的字符

### 切片

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];   // Зд
let s = &hello[0..3];   // panic
```

## Hash Map

`HashMap<K, V>`
- Key : Value
- Hash函数：决定如何的内存中存放K和V
- 适用场景：通过K（任何类型）来寻找数据，而不是通过索引
- `use std::collections::HashMap;`

注意：
- Heap 中
- 同构：Key同一种，Value同一种

### 创建，增加

- 创建：
    - `HashMap::new()` 函数
    - vector的`.collect()`方法
- 增加：`.insert()` 方法

```rust
use std::collections::HashMap;

let mut scores = HashMap::new(); // 根据后面插入的值自动推断出类型了

scores.insert(String::from("蓝队"), 10);
scores.insert(String::from("黄队"), 50);
```

```rust
use std::collections::HashMap;

let teams = vec![String::from("蓝队"), String::from("黄队")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
println!("{:?}", scores); // {"蓝队": 10, "黄队": 50}
```

### 所有权

- 实现了 Copy trait 的类（如i32），其值被拷贝到 HashMap
- 拥有所有权的值（如String），其值将被移动而 HashMap 会成为这些值的所有者
- 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map
    - 但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);    // 借用了移动的值，报错
```

### 访问

`.get(K: K)`方法，返回`Option<V>`

例子
```rust
use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("黄队"), 50);

    print_score(&scores, &"蓝队".to_string());
    print_score(&scores, &"红队".to_string());
    
}

fn print_score(scores: &HashMap<String, i32>, team_name: &String){
    match scores.get(team_name) {
        Some(s) => println!("{}的分数是{}", team_name, s),
        None => println!("无法找到{}的成绩", team_name),
    }
}
```

遍历 `for (K, V) in &HashMap`
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("蓝队"), 10);
scores.insert(String::from("黄队"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### 更新

- 大小可变
- 每个键只能关联一个值

如果 Key 已经存在
1. 替换现有值
2. 保留现有值，忽略新值
3. 合并

#### 替换

再次`.insert()`替换

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("蓝队"), 10);
scores.insert(String::from("蓝队"), 25);

println!("{:?}", scores);
```

#### 不存在才插入 .entry().or_insert()

`.entry()` 检查键是否有值
- 返回值是一个枚举，`Entry`，它代表了可能存在也可能不存在的键。
    - 键存在：`Entry(OccupiedEntry { key: "蓝队", value: 10 })`
    - 键不存在：`Entry(VacantEntry("黄队"))`
- Entry 有一个`or_insert()`方法
    - K 存在，返回到对应的 V 的一个可变引用
    - K 不存在，将方法参数作为K的新值插入，返回到这个值的可变引用

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("蓝队"), 10);

scores.entry(String::from("黄队")).or_insert(50);
scores.entry(String::from("蓝队")).or_insert(50);

println!("{:?}", scores);
```

`.or_insert()`
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
// {"world": 2, "hello": 1, "wonderful": 1}
```

### Hash 函数

HashMap 默认使用一种 “密码学安全的”（cryptographically strong）哈希函数
- 可以抵抗 DoS 攻击
- 并不是可用的最快的算法

如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。hasher 是一个实现了 BuildHasher trait 的类型。