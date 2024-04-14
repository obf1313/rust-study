fn main() {
    println!("Hello, world!");
}

fn test_lifetime() {
    // 悬垂指针和生命周期
    // 生命周期的主要作用是避免悬垂引用，它会导致程序引用了本不该引用的数据
    {
        // 此处 r 就是一个悬垂指针，它引用了提前被释放的变量 x
        // let r; 的声明方式貌似存在使用 null 的风险，实际上，当我们不初始化它就使用时，编译器会给予报错
        let r;
        {
            let x = 5;
            // r 引用了内部花括号中的 x 变量，但是 x 会在内部花括号 } 处被释放，因此回到外部花括号后，r 会引用一个无效的 x
            // FIXME
            r = &x;
        }
        println!("r: {}", r);
    }

    // 借用检查
    // 为了保证 Rust 的所有权和借用的正确性，Rust 使用了一个借用检查器(Borrow checker)，来检查我们程序的借用正确性
    {
        // r 变量被赋予了生命周期 'a
        let r;
        {
            // x 被赋予了生命周期 'b，生命周期 'b 比 'a 小很多
            let x = 5;
            r = &x;
        } // 生命周期 'b 结束
        println!("r: {}", r);
    }
    // 生命周期 'a 结束
    // Rust 会比较两个变量的生命周期，结果发现 r 明明拥有生命周期 'a，但是却引用了一个小得多的生命周期 'b，在这种情况下，编译器会认为我们的程序存在风险，因此拒绝运行
    // 如果想要编译通过，也很简单，只要 'b 比 'a 大就好。总之，x 变量只要比 r 活得久，那么 r 就能随意引用 x 且不会存在危险
    {
        // 'b
        let x = 5;
        // 'a
        let r = &x;
        println!("r: {}", r);
    }
    // 'b 'a 结束
    // 现在 x 的生命周期 'b 大于 r 的生命周期 'a，因此 r 对 x 的引用是安全的。

    // 函数中的生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // result 必须要活到 println!处，因为 result 的生命周期是 'a，因此 'a 必须持续到 println!
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let s = longest1("not", "important");
}

// 编译器无法知道该函数的返回值到底引用 x 还是 y ，因为编译器需要知道这些，来确保函数调用后的引用生命周期分析
// 在存在多个引用时，编译器有时会无法自动推导生命周期，此时就需要我们手动去标注，通过为参数标注合适的生命周期来帮助编译器进行借用检查的分析
// 和泛型一样，使用生命周期参数，需要先声明 <'a>
// x、y 和返回值至少活得和 'a 一样久(因为返回值要么是 x，要么是 y)
// 生命周期 'a 不代表生命周期等于 'a，而是大于等于 'a
// 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 标记的生命周期只是为了取悦编译器，让编译器不要难为我们
// 生命周期的语法也颇为与众不同，以 ' 开头，名称往往是一个单独的小写字母，大多数人都用 'a 来作为生命周期的名称。
// &i32        // 一个引用
// &'a i32     // 具有显式生命周期的引用
// &'a mut i32 // 具有显式生命周期的可变引用
fn useless<'a>(first: &'a i32, second: &'a i32) {}

// 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于
// 函数参数的生命周期
// 函数体中某个新建引用的生命周期
// 典型的悬垂引用场景
// fn longest1<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     // result 在函数结束后就被释放，但是在函数结束后，对 result 的引用依然在继续
//     result.as_str()
// }
// 那遇到这种情况该怎么办？最好的办法就是返回内部字符串的所有权，然后把字符串的所有权转移给调用者
fn longest1<'a>(_x: &str, _y: &str) -> String {
    String::from("really long string")
}

// 结构体中的生命周期
// 只要为结构体中的每一个引用标注上生命周期即可
// 结构体 ImportantExcerpt 所引用的字符串 str 生命周期需要大于等于该结构体的生命周期。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test_struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // 下面的代码就无法通过编译
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    // 结构体比它引用的字符串活得更久
    // println!("{:?}", i);
}

// 生命周期消除
// 对于编译器来说，每一个引用类型都有一个生命周期，那么为什么我们在使用过程中，很多时候无需标注生命周期？
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// 对于 first_word 函数，它的返回值是一个引用类型，那么该引用只有两种情况
// 从参数获取
// 从函数体内部新创建的变量获取
// 如果是后者，就会出现悬垂引用，最终被编译器拒绝
// 因此只剩一种情况：返回值的引用是获取自参数，这就意味着参数和返回值的生命周期是一样的。

// 函数或者方法中，参数的生命周期被称为 输入生命周期，返回值的生命周期被称为 输出生命周期

// 三条消除规则
// 1. 每一个引用参数都会获得独自的生命周期
// 例如一个引用参数的函数就有一个生命周期标注: fn foo<'a>(x: &'a i32)，两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推。
// 2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期
// 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期

// 方法中的生命周期
struct ImportantExcerpt1<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt1<'a> {
    fn level(&self) -> i32 {
        3
    }
}
// impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
// 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则
impl<'a> ImportantExcerpt1<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 首先，编译器应用第一规则，给予每个输入参数一个生命周期:
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// 接着，编译器应用第三规则，将 &self 的生命周期赋给返回值 &str
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// 将方法返回的生命周期改为'b
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str {
//         println!("Attention please: {}", announcement);
//         // 因为编译器无法知道 'a 和 'b 的关系。 &self 生命周期是 'a，那么 self.part 的生命周期也是 'a
//         // 但是好巧不巧的是，我们手动为返回值 self.part 标注了生命周期 'b，因此编译器需要知道 'a 和 'b 的关系
//         // 生命周期 'b 必须要比 'a 小
//         self.part
//     }
// }
// 'a: 'b，是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
// impl<'a: 'b, 'b> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// 可以把 'a 和 'b 都在同一个地方声明（如上），或者分开声明但通过 where 'a: 'b 约束生命周期关系，如下：
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期
// 在 Rust 中有一个非常特殊的生命周期，那就是 'static，拥有该生命周期的引用可以和整个程序活得一样久
// 字符串字面量，提到过它是被硬编码进 Rust 的二进制文件中
// 因此这些字符串变量全部具有 'static 的生命周期
fn test_static_lifetime() {
    let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
}
// 生命周期 'static 意味着能和程序活得一样久，例如字符串字面量和特征对象
// 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹
// 关于 'static, 有两种用法: &'static 和 T: 'static

// 一个复杂例子: 泛型、特征约束
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
