// 基于 Send 和 Sync 的线程安全
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    println!("Hello, world!");
}
// 为何 Rc、RefCell 和裸指针不可以在多线程间使用？如何让裸指针可以在多线程使用？我们一起来探寻下这些问题的答案。
// 无法用于多线程的Rc
// fn test_rc() {
//     let v = Rc::new(5);
//     // the trait `Send` is not implemented for `Rc<i32>`
//     let t = thread::spawn(move || {
//         println!("{}", v);
//     });

//     t.join().unwrap();
// }

// Rc 和 Arc 源码对比
// Arc为何可以在多线程使用，玄机在于两者的源码实现上
// Rc源码片段
// !代表移除特征的相应实现
// Rc<T>的Send和Sync特征被特地移除了实现
// impl<T: ?Sized> !marker::Send for Rc<T> {}
// impl<T: ?Sized> !marker::Sync for Rc<T> {}

// Arc源码片段
// 而Arc<T>则相反，实现了Sync + Send
// unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
// unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}

// Send和Sync是在线程间安全使用一个值的关键。

// Send 和 Sync
// 几乎所有类型都默认实现了Send和Sync，而且由于这两个特征都是可自动派生的特征(通过derive派生)
// 意味着一个复合类型(例如结构体), 只要它内部的所有成员都实现了Send或者Sync，那么它就自动实现了Send或Sync。
// 除了以下几个（事实上不止这几个，只不过它们比较常见）
// 1. 裸指针两者都没实现，因为它本身就没有任何安全保证
// 2. UnsafeCell不是Sync，因此Cell和RefCell也不是
// 3. Rc两者都没实现(因为内部的引用计数器不是线程安全的)
// 当然，如果是自定义的复合类型，那没实现那哥俩的就较为常见了：只要复合类型中有一个成员不是Send或Sync，那么该复合类型也就不是Send或Sync。
// 手动实现 Send 和 Sync 是不安全的，通常并不需要手动实现 Send 和 Sync trait，实现者需要使用unsafe小心维护并发安全保证。

// 为裸指针实现Send
// #[derive(Debug)]
// struct MyBox(*mut u8);
// // 手动实现 Send
// // Send和Sync是unsafe特征，实现时需要用unsafe代码块包裹。
// unsafe impl Send for MyBox {}

// fn test_send() {
//     let p = MyBox(5 as *mut u8);
//     let t = thread::spawn(move || {
//         println!("{:?}", p);
//     });

//     t.join().unwrap();
// }

// 为裸指针实现Sync
#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

fn test_sync() {
    let b = &MyBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });

    t.join().unwrap();
}

// 总结
// 1. 实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)
// 2. 绝大部分类型都实现了Send和Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc 等
// 3. 可以为自定义类型实现Send和Sync，但是需要unsafe代码块
// 4. 可以为部分 Rust 中的类型实现Send、Sync，但是需要使用newtype，例如文中的裸指针例子
