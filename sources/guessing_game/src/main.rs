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
