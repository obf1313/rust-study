// 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
// 引用必须总是有效的

fn main() {
    reference();
    im_mute_reference();
    mute_reference();
    end_time();
    // let reference_to_nothing = dangle();
}

// 引用与解引用
fn reference() {
    let x = 5;
    // y 是 x 的一个引用
    let y = &x;

    assert_eq!(5, x);
    // * 解引用运算符
    // 使用解引用运算符来解出 y 所使用的值:
    assert_eq!(5, *y);
    // 报错
    // assert_eq!(5, y);
}

// s 是对 String 的引用
fn calculate_length(s: &String) -> usize {
    s.len()
}
// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生

// 不可变引用
fn im_mute_reference() {
    let s1 = String::from("hello");
    // 我们用 s1 的引用作为参数传递给 calculate_length 函数，而不是把 s1 的所有权转移给该函数
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

// 可变引用
fn mute_reference() {
    // 引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方
    // 这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号
    let mut s = String::from("hello");
    change(&mut s);
    // 同一作用域，特定数据只能有一个可变引用
    // 通过手动限制变量的作用域，可以帮我们解决一些编译不通过的问题
    {
        let r1 = &mut s;
    }
    // 第一个可变借用 r1 必须要持续到最后一次使用的位置 println!，在 r1 创建和最后一次使用之间，我们又尝试创建第二个可变借用 r2。
    let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // 可变引用与不可变引用不能同时存在
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s;
    // println!("{}, {}, and {}", s1, s2, s3);
}

// 引用指向的值默认也是不可变的
// fn change(some_string: &String) {
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}

// NLL
// 对于这种编译器优化行为，Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL)，专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。
fn end_time() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1，r2 作用域在这里结束
    let r3 = &mut s;
    println!("{}", r3);
}
// 老编译器中，r1、r2、r3作用域在这里结束
// 新编译器中，r3作用域在这里结束

// 悬垂指针
// 意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用
// dangle 返回一个字符串的引用
// fn dangle() -> &String {
//     // s 是一个新字符串
//     let s = String::from("hello");
//     // 返回字符串 s 的引用
//     &s
// }
// 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！

// 其中一个很好的解决方法是直接返回 String
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
