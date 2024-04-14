use std::{fs::File, io::ErrorKind};

fn main() {
    println!("Hello, world!");
    // File::open 返回一个 Result 类型
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    // 对返回的错误进行处理
    let f = match f {
        Ok(file) => file,
        // 对 error 进行了详细的匹配解析
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 失败就 panic: unwrap 和 expect
    // 如果返回成功，就将 Ok(T) 中的值取出来，如果失败，就直接 panic
    let f = File::open("hello.txt").unwrap();
    // expect 跟 unwrap 很像，也是遇到错误直接 panic, 但是会带上自定义的错误提示信息，相当于重载了错误打印的函数
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // 带返回值的 main 函数
    // 因为 ? 要求 Result<T, E> 形式的返回值，而 main 函数的返回是 ()
    // let f = File::open("hello.txt")?;
    // 实际上 Rust 还支持另外一种形式的 main 函数
    // 至于 main 函数可以有多种返回值，那是因为实现了 std::process::Termination 特征
    // fn main() -> Result<(), Box<dyn std::error::Error>> {
    //     let f = File::open("hello.txt")?;
    //     Ok(())
    // }
}
// 泛型参数 T 代表成功时存入的正确值的类型，存放方式是 Ok(T)，E 代表错误时存入的错误值，存放方式是 Err(E)

// 传播错误
use std::io::{self, Read};

// 该函数返回一个 Result<String, io::Error> 类型，当读取用户名成功时，返回 Ok(String)，失败时，返回 Err(io:Error)
// fn read_username_from_file() -> Result<String, io::Error> {
//     // 打开文件，f是`Result<文件句柄,io::Error>`
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         // 打开文件成功，将file句柄赋值给f
//         Ok(file) => file,
//         // 打开文件失败，将错误返回(向上传播)
//         // File::open 和 f.read_to_string 返回的 Result<T, E> 中的 E 就是 io::Error
//         Err(e) => return Err(e),
//     };
//     // 创建动态字符串s
//     let mut s = String::new();
//     // 从f文件句柄读取数据并写入s中
//     match f.read_to_string(&mut s) {
//         // 读取成功，返回Ok封装的字符串
//         Ok(_) => Ok(s),
//         // 将错误向上传播
//         // File::open 和 f.read_to_string 返回的 Result<T, E> 中的 E 就是 io::Error
//         Err(e) => Err(e),
//     }
// }

// 传播界的大明星: ?
// fn read_username_from_file() -> Result<String, io::Error> {
//     // 其实 ? 就是一个宏，它的作用跟上面的 match 几乎一模一样
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     // 其实 ? 就是一个宏，它的作用跟上面的 match 几乎一模一样
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    // File::open 报错时返回的错误是 std::io::Error 类型
    // 但是 open_file 函数返回的错误类型是 std::error::Error 的特征对象
    // 根本原因是在于标准库中定义的 From 特征，该特征有一个方法 from，用于把一个类型转成另外一个类型
    // ? 可以自动调用该方法，然后进行隐式类型转换
    // // 这种转换非常好用，意味着你可以用一个大而全的 ReturnError 来覆盖所有错误类型，只需要为各种子错误类型实现这种转换即可
    let mut f = File::open("hello.txt")?;
    Ok(f)
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     // ? 还能实现链式调用
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

use std::fs;
fn read_username_from_file() -> Result<String, io::Error> {
    // read_to_string是定义在std::io中的方法，因此需要在上面进行引用
    fs::read_to_string("hello.txt")
}

// ? 用于 Option 的返回
// ? 不仅仅可以用于 Result 的传播，还能用于 Option 的传播
// Option 通过 ? 返回 None
// fn first(arr: &[i32]) -> Option<&i32> {
//     // 如果 get 的结果是 None，则直接返回 None，如果是 Some(&i32)，则把里面的值赋给 v
//     let v = arr.get(0)?;
//     Some(v)
// }

fn first(arr: &[i32]) -> Option<&i32> {
    arr.get(0)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 新手用 ? 常会犯的错误
// fn first(arr: &[i32]) -> Option<&i32> {
// 这段代码无法通过编译，切记：? 操作符需要一个变量来承载正确的值
// 这个函数只会返回 Some(&i32) 或者 None，只有错误值能直接返回，正确的值不行
// 所以如果数组中存在 0 号元素，那么函数第二行使用 ? 后的返回类型为 &i32 而不是 Some(&i32)
// arr.get(0)?
// }
// 因此 ? 只能用于以下形式
// let v = xxx()?;
// xxx()?.yyy()?;

// try!
// 在 ? 横空出世之前( Rust 1.13 )，Rust 开发者还可以使用 try! 来处理错误，该宏的大致定义如下
// 在当前版本中，我们要尽量避免使用 try!
// macro_rules! try {
//     ($e:expr) => (match $e {
//         Ok(val) => val,
//         Err(err) => return Err(::std::convert::From::from(err)),
//     });
// }
