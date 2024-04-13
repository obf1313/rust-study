use std::fmt;
use std::ops::Add;

fn main() {
    let mut c = Counter { name: 50 };
    // 对于 next 方法而言，Self 是调用者 c 的具体类型： Counter
    // 而 Self::Item 是 Counter 中定义的 Item 类型: u32
    c.next();
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    test_baby_name();
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

struct Counter {
    name: i32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(20)
    }
}

// 关联类型
pub trait Iterator {
    type Item;

    // Self 用来指代当前调用者的具体类型，那么 Self::Item 就用来指代该类型实现中定义的 Item 类型
    fn next(&mut self) -> Option<Self::Item>;
}

// 为了代码的可读性，当你使用了泛型后，你需要在所有地方都写 Iterator<Item>
// // 而使用了关联类型，你只需要写 Iterator
// pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
//     type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
//     fn is_null(&self) -> bool;
// }

// 由于使用了泛型，导致函数头部也必须增加泛型的声明
// trait Container<A, B> {
//     fn contains(&self, a: A, b: B) -> bool;
// }
// fn difference<A, B, C>(container: &C) -> i32
// where
//     C: Container<A, B>,
// {
//     1
// }
// 而使用关联类型，将得到可读性好得多的代码
trait Container {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

fn difference<C: Container>(container: &C) {}

// 默认泛型类型参数
// 当使用泛型类型参数时，可以为其指定一个默认的具体类型
// trait Add<RHS = Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 运算符重载
// 只有定义在 std::ops 中的运算符才能进行重载
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// 使用 Add<Meters> 可以将 RHS 指定为 Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 调用同名的方法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
// 优先调用类型上的方法
fn test_fly() {
    let person = Human;
    // 调用Human类型自身的方法
    person.fly();
    // 调用特征上的方法
    // 调用Pilot特征上的方法
    Pilot::fly(&person);
    // 调用Pilot特征上的方法
    Wizard::fly(&person);
}

// 如果方法没有 self 参数
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn test_baby_name() {
    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name());
    // 完全限定语法
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// 特征定义中的特征约束
// 我们会需要让某个特征 A 能使用另一个特征 B 的功能(另一种形式的特征约束)
// 这种情况下，不仅仅要为类型实现特征 A，还要为类型实现特征 B 才行
use std::fmt::Display;

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// 在外部类型上实现外部特征(newtype)
// 绕过孤儿规则，那就是使用newtype 模式
// 就是为一个元组结构体创建新类型

// struct Wrapper(Vec<String>) 就是一个元组结构体
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Rust 提供了一个特征叫 Deref，实现该特征后，可以自动做一层类似类型转换的操作
        // 可以将 Wrapper 变成 Vec<String> 来使用
        // 这样就会像直接使用数组那样去使用 Wrapper，而无需为每一个操作都添加上 self.0。
        write!(f, "[{}]", self.0.join(", "))
    }
}
