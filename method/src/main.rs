fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{}", rect1.width());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数
    // 因为是函数，所以不能用 . 的方式来调用，我们需要用 :: 来调用
    // 例如：let sq = Rectangle::new(3, 3);
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
    // self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
    // &self 表示该方法对 Rectangle 的不可变借用
    // &mut self 表示可变借用
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 方法名跟结构体字段名相同
    // fn width(&self) -> bool {
    //     self.width > 0
    // }
    // 一般来说，方法跟字段同名，往往适用于实现 getter 访问器，例如
    pub fn width(&self) -> u32 {
        return self.width;
    }

    // 带有多个参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Rust 允许我们为一个结构体定义多个 impl 块
impl Rectangle {
    fn test(&self, other: &Rectangle) {
        println!("just a test")
    }
}

// 为枚举实现方法
fn enum_method() {
    #![allow(unused)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}
