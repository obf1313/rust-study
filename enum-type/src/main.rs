use std::net::TcpStream;

fn main() {
    enum_type();
    test_plus_one();
}
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

fn enum_type() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond)
}

fn print_suit(card: PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否侧会报 card 没有实现 Debug
    println!("{:?}", card);
}

fn poker_card() {
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
}

fn poker_card_enum() {
    // 任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举
    enum PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(char),
        Hearts(char),
    }
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds('A');

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);
}

// 当然，我们也可以用结构体的方式来定义这些消息：
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
// 元组结构体
struct WriteMessage(String);
// 元组结构体
struct ChangeColorMessage(i32, i32, i32);
// 由于每个结构体都有自己的类型，因此我们无法在需要同一类型的地方进行使用，例如某个函数它的功能是接受消息并进行发送，
// 那么用枚举的方式，就可以接收不同的消息，但是用结构体，该函数无法接受 4 个不同的结构体作为参数。

// Option 枚举用于处理空值
// prelude: Rust 标准库，Rust 会将最常用的类型、函数等提前引入其中
enum Options<T> {
    Some(T),
    None,
}

fn test_options() {
    let some_number = Some(5);
    let some_string = Some("a string");
    // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
    let absent_number: Option<i32> = None;
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
