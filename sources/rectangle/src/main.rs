fn main() {
    let rec1 = Rectangle { width: 30, height: 50 };

    println!("The area of this rectangle is {}.", area(&rec1));

    let rec2 = Rectangle {
        width: 50,
        ..rec1
    };

    println!("The area of this rectangle1 is {}.", area(&rec2));

    println!("The area of this rectangle2 is {}.", rec2.area());

    println!("Can rec1 hold rec2? {}.", rec1.can_hold(&rec2));

    let squ1 = Rectangle::square(30);
    println!("{:#?}", squ1);
}

#[derive(Debug)]    // 增加注解来派生 Debug trait
struct Rectangle {
    width : u32,
    height : u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数，构造一个正方形
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}