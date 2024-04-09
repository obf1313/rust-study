fn main() {
    println!("Hello, world!");
    test_match();

    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);

    test_action();

    // 变量遮蔽
    let age = Some(30);
    println!("在匹配前，age 是{:?}", age);
    if let Some(age) = age {
        println!("在匹配后，age 是{:?}", age);
    }
    // 可以看出在 if let 中，= 右边 Some(i32) 类型的 age 被左边 i32 类型的新 age 遮蔽了，该遮蔽一直持续到 if let 语句块的结束。
    // 因此第三个 println! 输出的 age 依然是 Some(i32) 类型。
    println!("在匹配后，age 是{:?}", age);
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn test_match() {
    let dire = Direction::South;
    // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    // match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
    // X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("North or South")
        }
        // _ 类似 switch 的 default
        // 通配符
        _ => println!("West"),
    };

    // match 本身也是一个表达式，因此可以用它来赋值
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str)

    // 模式绑定
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // 25美分硬币
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn test_action() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => println!("{}", s),
            Action::MoveTo(x, y) => println!("Move to {}, {}", x, y),
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

// 只关心某一个值是否存在
// if let
// 那不就是个普通 if 吗
fn test_if_let() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// matches!宏
enum MyEnum {
    Foo,
    Bar,
}

fn test_my_enum() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    v.iter().filter(|x| matches!(x, MyEnum::Foo));
}
