use std::fmt::Display;

fn main() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    test_trait();

    // println!("{}", post.summarize());
    // println!("{}", weibo.summarize());
}
// 特征定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为
// 定义特征
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// 为类型实现特征
pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者是{}", self.title, self.author)
//     }
// }

pub struct Weibo {
    pub username: String,
    pub content: String,
}

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// 特征定义与实现的位置(孤儿规则)
// 如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！

// 默认实现
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// impl Summary for Post {}

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// 默认实现允许调用相同特征中的其他方法
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 使用特征作为函数参数
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// 特征约束(trait bound)
// 形如 T: Summary 被称为特征约束
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 一个函数接受两个 impl Summary 的参数
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// 多重约束
// pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

// Where 约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// 使用特征约束有条件地实现方法或特征
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // cmp_display 方法，并不是所有的 Pair<T> 结构体对象都可以拥有
    // 只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 也可以有条件地实现特征
// impl<T: Display> ToString for T {
//     // --snip--
// }

// 函数返回中的 impl Trait
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from("m1 max太厉害了，电脑再也不会卡"),
    }
}
// 但是这种返回值方式有一个很大的限制：只能有一个具体的类型
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Post {
//             title: String::from("Penguins win the Stanley Cup Championship!"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Weibo {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//         }
//     }
// }

// 通过 derive 派生特征
// derive 派生出来的是 Rust 默认给我们提供的特征
// #[derive(Debug)]

// 调用方法需要引入特征
// 但是 Rust 又提供了一个非常便利的办法，即把最常用的标准库中的特征通过 std::prelude 模块提前引入到当前作用域中，其中包括了 std::convert::TryInto
fn test_trait() {
    let a: i32 = 10;
    let b: u16 = 100;
    // 如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
