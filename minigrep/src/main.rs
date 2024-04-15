use std::env;
use std::fs;

fn main() {
    // 在程序中读取传入的参数
    // env::args 读取到的参数中第一个就是程序的可执行路径名
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    // dbg! 宏来输出读取到的数组内容
    // dbg!(args);
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
