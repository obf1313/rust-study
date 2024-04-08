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
fn main() {
    println!("Hello, world!");
    test_match()
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

// TODO: https://course.rs/basic/match-pattern/match-if-let.html
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
