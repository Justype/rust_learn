fn main() {
    // let mut g1 = "\nHello".to_string();

    // g1.push_str(" Rust");
    // g1.push('!');

    // let g2 = "\nHello World!".to_string();

    // let g3 = g1 + &g2;

    // // println!("greeting : {}", g1);  // 无法继续使用
    // println!("greeting : {}", g2); // 可以
    // println!("greeting : {}", g3);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{}-{}-{}", s1, s2, s3); // s1 所有权没有丢失
    // println!("{}", s);

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);

    let w = "नमस्ते";
    
    // bytes
    let mut bytes = "[".to_string();
    for (i, b) in w.bytes().enumerate() {
        if i == 0 {
            bytes += &format!(" {}", b);
        } else {
            bytes += &format!(", {}", b);
        }
    }
    bytes += " ]";
    println!("字节：{}", bytes);

    // chars
    let mut chars = "[".to_string();
    for (i, b) in w.chars().enumerate() {
        if i == 0 {
            chars += &format!(" {} ", b);
        } else {
            chars += &format!(", {} ", b);
        }
    }
    chars += " ]";
    println!("Unicode 标量值：{}", chars);
}
