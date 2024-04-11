fn main() {
    match_variable();
    match_destructure();
    match_array();
    foo(3, 4);
    ignore();
}

// 匹配字面量
fn match_text() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 匹配命名变量
fn match_variable() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 变量遮蔽
        // 可以换个名字或者 匹配守卫(match guard)
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// 单分支多模式
fn match_multiple() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 通过序列..=匹配值的范围
// 序列只允许用于数字或字符类型，原因是：它们可以连续
fn match_range() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

// 解构结构体
struct Point {
    x: i32,
    y: i32,
}

fn match_destructure() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 解构枚举
fn match_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }
}

// 解构嵌套的结构体和枚举
fn match_nested() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

// 解构结构体和元组
fn match_tuple() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

// 解构数组
fn match_array() {
    let [a, b, c] = [1, 2, 3];
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);
    // 不定长数组
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }
    let arr: &[u16] = &[];
    // [..] 是啥，空数组吗？
    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));

    println!("{:?}", [..]);
}

// 忽略模式中的值
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

// 使用嵌套的_忽略部分值
fn ignore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        // 通过_忽略不需要的值
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // 使用下划线开头忽略未使用的变量
    let _x = 5;
    let y = 10;

    // _x 仍会将值绑定到变量，而 _ 则完全不会绑定
    let s = Some(String::from("Hello"));
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // 用 .. 忽略剩余值
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // 还可以用 .. 来忽略元组中间的某些值
    // 使用 .. 必须是无歧义的
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

// 匹配守卫提供的额外条件
fn match_guard() {
    let num = Some(4);
    match num {
        // 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 使用匹配守卫修复变量覆盖问题
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @绑定
fn at_binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

// @前绑定后解构
fn at_binding_after_destructure() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}
