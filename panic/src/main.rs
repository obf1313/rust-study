fn main() {
    println!("Hello, world!");
    test_panic();
    // panic!("crash and burn");
}
// 对于这些严重到影响程序运行的错误，触发 panic 是很好的解决方式。
// 在 Rust 中触发 panic 有两种方式：被动触发和主动调用

// 被动触发
fn test_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

// 主动调用
// panic! 宏
// 当调用执行该宏时，程序会打印出一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序。
// panic!("crash and burn");

// backtrace 栈展开
// 现在成功知道问题发生的位置，但是如果我们想知道该问题之前经过了哪些调用环节
// RUST_BACKTRACE=1 cargo run 或 $env:RUST_BACKTRACE=1 ; cargo run

// panic 时的两种终止方式
// 栈展开和直接终止
// 默认的方式就是 栈展开，这意味着 Rust 会回溯栈上数据和函数调用，因此也意味着更多的善后工作，好处是可以给出充分的报错信息和栈调用信息，便于事后的问题复盘
// 直接终止，顾名思义，不清理数据就直接退出程序，善后工作交与操作系统来负责。
// 但是当你关心最终编译出的二进制可执行文件大小时，那么可以尝试去使用直接终止的方式，例如下面的配置修改 Cargo.toml 文件，实现在 release 模式下遇到 panic 直接终止
// [profile.release]
// panic = 'abort'
// FIXME: 配了好像没什么用

// 线程 panic 后，程序是否会终止？
// 长话短说，如果是 main 线程，则程序会终止，如果是其它子线程，该线程会终止，但是不会影响 main 线程。

// 何时该使用 panic!
// 1. 你确切的知道你的程序是正确时，可以使用 panic
// 2. 可能导致全局有害状态时
// 有害状态大概分为几类：
// 非预期的错误
// 后续代码的运行会受到显著影响
// 内存安全的问题
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// 当没有错误发生时，函数返回一个用 Result 类型包裹的值 Ok(T)
// 当错误时，返回一个 Err(E)
// 对于 Result 返回我们有很多处理方法，最简单粗暴的就是 unwrap 和 expect
use std::net::IpAddr;
fn test_result() {
    // parse 方法试图将字符串 "127.0.0.1" 解析为一个 IP 地址类型 IpAddr，它返回一个 Result<IpAddr, E> 类型
    // 如果解析成功，则把 Ok(IpAddr) 中的值赋给 home，如果失败，则不处理 Err(E)，而是直接 panic
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    // 因此 unwrap 简而言之：成功则返回值，失败则 panic，总之不进行任何错误处理。
}

// panic 原理剖析
// 当调用 panic! 宏时，它会
// 1. 格式化 panic 信息，然后使用该信息作为参数，调用 std::panic::panic_any() 函数
// 2. panic_any 会检查应用是否使用了 panic hook，如果使用了，该 hook 函数就会被调用（hook 是一个钩子函数，是外部代码设置的，用于在 panic 触发时，执行外部代码所需的功能）
// 3. 当 hook 函数返回后，当前的线程就开始进行栈展开：从 panic_any 开始，如果寄存器或者栈因为某些原因信息错乱了，那很可能该展开会发生异常，最终线程会直接停止，展开也无法继续进行
// 4. 展开的过程是一帧一帧的去回溯整个栈，每个帧的数据都会随之被丢弃，但是在展开过程中，你可能会遇到被用户标记为 catching 的帧（通过 std::panic::catch_unwind() 函数标记），此时用户提供的 catch 函数会被调用，展开也随之停止：当然，如果 catch 选择在内部调用 std::panic::resume_unwind() 函数，则展开还会继续。
// 还有一种情况，在展开过程中，如果展开本身 panic 了，那展开线程会终止，展开也随之停止。
// 一旦线程展开被终止或者完成，最终的输出结果是取决于哪个线程 panic：对于 main 线程，操作系统提供的终止功能 core::intrinsics::abort() 会被调用，最终结束当前的 panic 进程；如果是其它子线程，那么子线程就会简单的终止，同时信息会在稍后通过 std::thread::join() 进行收集。
