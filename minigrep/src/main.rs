use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // 在程序中读取传入的参数
    // env::args 读取到的参数中第一个就是程序的可执行路径名
    let args: Vec<String> = env::args().collect();
    // unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值
    // 如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // 当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程
        process::exit(1);
        // 不太懂怎么在报错
    });
    // dbg! 宏来输出读取到的数组内容
    // dbg!(args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    // run(config);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
