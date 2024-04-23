use std::{ops::Deref, rc::Rc};

fn main() {
    println!("Hello, world!");
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    // self 是 &mut Person 的类型
    fn display(self: &mut Person, age: u8) {
        // 对其取了一次引用 &self
        // 此时 &self 的类型是 &&mut Person
        let Person { name, age } = &self;
    }
}

// 智能指针的名称来源，主要就在于它实现了 Deref 和 Drop 特征，这两个特征可以智能地帮助我们节省使用上的负担
// 1. Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
// 2. Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

fn test_deref() {
    // 通过 * 获取引用背后的值
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 智能指针解引用
    let x = Box::new(1);
    let sum = *x + 1;

    // 自己的智能指针
    let y = MyBox::new(5);
    // 实现 Deref 特征
    assert_eq!(5, *y);
}

// 定义自己的智能指针
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     // 在 Deref 特征中声明了关联类型 Target，在之前章节中介绍过，关联类型主要是为了提升代码可读性
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         // deref 返回的是一个常规引用，可以被 * 进行解引用
//         &self.0
//     }
// }

// * 背后的原理
// 当我们对智能指针 Box 进行解引用时，实际上 Rust 为我们调用了以下方法：
// *(y.deref())
// * 不会无限递归替换，从 *y 到 *(y.deref()) 只会发生一次，而不会继续进行替换然后产生形如 *((y.deref()).deref()) 的怪物

// 函数和方法中的隐式 Deref 转换
fn test_display() {
    // String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
    let s = String::from("hello world");
    // 若一个类型实现了 Deref 特征，那它的引用在传给函数或方法时，会根据参数签名来决定是否进行隐式的 Deref 转换
    // &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
    // 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)
    display(&s)
}
fn display(s: &str) {
    println!("{}", s);
}

// 连续的隐式 Deref 转换
// Deref 可以支持连续的隐式转换，直到找到适合的形式为止：
fn test_display1() {
    // 首先 MyBox 被 Deref 成 String 类型，结果并不能满足 display 函数参数的要求，编译器发现 String 还可以继续 Deref 成 &str，最终成功的匹配了函数参数
    let s = MyBox::new(String::from("hello world"));
    display(&s);

    let s = MyBox::new(String::from("hello, world"));
    // 对于 s1，我们通过两次 Deref 将 &str 类型的值赋给了它（赋值操作需要手动解引用）；
    let s1: &str = &s;
    // 而对于 s2，我们在其上直接调用方法 to_string，实际上 MyBox 根本没有没有实现该方法，能调用 to_string，完全是因为编译器对 MyBox 应用了 Deref 的结果（方法调用会自动解引用）。
    let s2: String = s.to_string();
}

fn display1(s: &str) {
    println!("{}", s);
}

// Deref 规则总结
// 一个类型为 T 的对象 foo，如果 T: Deref<Target=U>，那么，相关 foo 的引用 &foo 在应用的时候会自动转换为 &U。
// 引用归一化
// Rust 会在解引用时自动把智能指针和 &&&&v 做引用归一化操作，转换成 &v 形式，最终再对 &v 进行解引用
// 把智能指针（比如在库中定义的，Box、Rc、Arc、Cow 等）从结构体脱壳为内部的引用类型，也就是转成结构体内部的 &v
// 把多重&，例如 &&&&&&&v，归一成 &v
fn foo(s: &str) {}
fn test_deref1() {
    // 由于 String 实现了 Deref<Target=str>
    let owned = "Hello".to_string();
    // 因此下面的函数可以正常运行：
    foo(&owned);
    // String 实现了 Deref<Target=str>
    let owned = "Hello".to_string();
    // 且 Rc 智能指针可以被自动脱壳为内部的 `owned` 引用： &String ，然后 &String 再自动解引用为 &str
    let counted = Rc::new(owned);
    // 因此下面的函数可以正常运行:
    foo(&counted);
}

// 三种 Deref 转换
// 当 T: Deref<Target=U>，可以将 &T 转换成 &U，也就是我们之前看到的例子
// 当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
// 当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

use std::ops::DerefMut;

// 要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
// T: DerefMut<Target=U> 解读：将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，对应上例中，就是将 &mut MyBox<String> 转换为 &mut String
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn test_main() {
    let mut s = MyBox::new(String::from("hello, "));
    test_display2(&mut s)
}

fn test_display2(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}
