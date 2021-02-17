fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * 解引用
    }
}
