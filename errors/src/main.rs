use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, Read};
use std::num;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // match produce_error() {
    //     Err(e) => eprintln!("{}", e),
    //     _ => println!("No error"),
    // }

    // eprintln!("{:?}", produce_error()); // Err({ file: src/main.rs, line: 17 })
    // eprintln!("{:#?}", produce_error());

    // ? 可以将错误进行隐式的强制转换：File::open 返回的是 std::io::Error
    // let _file = File::open("nonexistent_file.txt")?;
    // Ok(())

    // let mut file = File::open("hello_world.txt")?;

    // let mut content = String::new();
    // file.read_to_string(&mut content)?;

    // let _number: usize;
    // _number = content.parse()?;

    // Ok(())

    let html = render()?;
    println!("{}", html);
    Ok(())
}

// 组合器
// or() 和 and()
// or()，表达式按照顺序求值，若任何一个表达式的结果是 Some 或 Ok，则该值会立刻返回
// and()，若两个表达式的结果都是 Some 或 Ok，则第二个表达式中的值被返回。若任何一个的结果是 None 或 Err ，则立刻返回。
fn test_and_or() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");

    assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
    assert_eq!(s1.or(n), s1); // Some or None = Some
    assert_eq!(n.or(s1), s1); // None or Some = Some
    assert_eq!(n.or(n), n); // None1 or None2 = None2

    assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
    assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
    assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

    assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
    assert_eq!(s1.and(n), n); // Some and None = None
    assert_eq!(n.and(s1), n); // None and Some = None
    assert_eq!(n.and(n), n); // None1 and None2 = None1

    assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
    assert_eq!(o1.and(e1), e1); // Ok and Err = Err
    assert_eq!(e1.and(o1), e1); // Err and Ok = Err
    assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
}
// Rust 还为我们提供了 xor ，但是它只能应用在 Option 上，其实想想也是这个理

// or_else() 和 and_then()
fn test_or_else() {
    // or_else with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
    assert_eq!(s1.or_else(fn_none), s1); // Some or_else None = Some
    assert_eq!(n.or_else(fn_some), s2); // None or_else Some = Some
    assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

    // or_else with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
    assert_eq!(e1.or_else(fn_ok), o2); // Err or_else Ok = Ok
    assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2
}

fn test_and_then() {
    // and_then with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = |_| Some("some2"); // 类似于: let fn_some = |_| -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = |_| None;

    assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none), n); // Some and_then None = None
    assert_eq!(n.and_then(fn_some), n); // None and_then Some = None
    assert_eq!(n.and_then(fn_none), n); // None1 and_then None2 = None1

    // and_then with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = Ok2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
}

// filter
// filter 用于对 Option 进行过滤
fn test_filter() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;

    assert_eq!(s1.filter(fn_is_even), n); // Some(3) -> 3 is not even -> None
    assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
    assert_eq!(n.filter(fn_is_even), n); // None -> no value -> None
}

// map() 和 map_err()
fn test_map() {
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");

    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(fn_character_count), s2); // Some1 map = Some2
    assert_eq!(n1.map(fn_character_count), n2); // None1 map = None2

    assert_eq!(o1.map(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map(fn_character_count), e2); // Err1 map = Err2
}
// 但是如果你想要将 Err 中的值进行改变， map 就无能为力了，此时我们需要用 map_err
fn test_map_err() {
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2
}

// map_or() 和 map_or_else()
// map_or 在 map 的基础上提供了一个默认值
fn test_map_or() {
    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;
    // 当处理 None 的时候，V_DEFAULT 作为默认值被直接返回。
    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);
}
// map_or_else 与 map_or 类似，但是它是通过一个闭包来提供默认值:
fn test_map_or_else() {
    let s = Some(10);
    let n: Option<i8> = None;

    let fn_closure = |v: i8| v + 2;
    let fn_default = || 1;

    assert_eq!(s.map_or_else(fn_default, fn_closure), 12);
    assert_eq!(n.map_or_else(fn_default, fn_closure), 1);

    let o = Ok(10);
    let e = Err(5);
    let fn_default_for_result = |v: i8| v + 1; // 闭包可以对 Err 中的值进行处理，并返回一个新值

    assert_eq!(o.map_or_else(fn_default_for_result, fn_closure), 12);
    assert_eq!(e.map_or_else(fn_default_for_result, fn_closure), 6);
}

// ok_or() and ok_or_else()
// 这两兄弟可以将 Option 类型转换为 Result 类型。其中 ok_or 接收一个默认的 Err 参数
fn test_ok_or() {
    const ERR_DEFAULT: &str = "error message";

    let s = Some("abcde");
    let n: Option<&str> = None;

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err(ERR_DEFAULT);

    assert_eq!(s.ok_or(ERR_DEFAULT), o); // Some(T) -> Ok(T)
    assert_eq!(n.ok_or(ERR_DEFAULT), e); // None -> Err(default)
}
fn test_ok_or_else() {
    let s = Some("abcde");
    let n: Option<&str> = None;
    let fn_err_message = || "error message";

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err("error message");

    assert_eq!(s.ok_or_else(fn_err_message), o); // Some(T) -> Ok(T)
    assert_eq!(n.ok_or_else(fn_err_message), e); // None -> Err(default)
}

// 自定义错误类型
// Rust 在标准库中提供了一些可复用的特征，例如 std::error::Error 特征
// 实际上，自定义错误类型只需要实现 Debug 和 Display 特征即可，source 方法是可选的，而 Debug 特征往往也无需手动实现，可以直接通过 derive 来派生

// pub trait Error: Debug + Display {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {}
// }

// 最简单的错误
// AppError 是自定义错误类型，它可以是当前包中定义的任何类型，在这里为了简化，我们使用了单元结构体作为例子。
// 为 AppError 自动派生 Debug 特征
// #[derive(Debug)]
// struct AppError;

// // 为 AppError 实现 std::fmt::Display 特征
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "An Error Occurred, Please Try Again!") // user-facing output
//     }
// }

// // 一个示例函数用于产生 AppError 错误
// fn produce_error() -> Result<(), AppError> {
//     Err(AppError)
// }
// 事实上，实现 Debug 和 Display 特征并不是作为 Err 使用的必要条件
// 为何要为自定义类型实现这两个特征
// 1. 错误得打印输出后，才能有实际用处，而打印输出就需要实现这两个特征
// 2. 可以将自定义错误转换成 Box<dyn std::error:Error> 特征对象，在后面的归一化不同错误类型部分，我们会详细介绍

// 更详尽的错误
// 再来定义一个具有错误码和信息的错误
// struct AppError {
//     code: usize,
//     message: String,
// }

// // 根据错误码显示不同的错误信息
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let err_msg = match self.code {
//             404 => "Sorry, Can not find the Page!",
//             _ => "Sorry, something is wrong! Please Try Again!",
//         };

//         write!(f, "{}", err_msg)
//     }
// }

// // 在本例中，我们除了增加了错误码和消息外，还手动实现了 Debug 特征
// // 原因在于，我们希望能自定义 Debug 的输出内容，而不是使用派生后系统提供的默认输出形式。
// impl fmt::Debug for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "AppError {{ code: {}, message: {} }}",
//             self.code, self.message
//         )
//     }
// }

// fn produce_error() -> Result<(), AppError> {
//     Err(AppError {
//         code: 404,
//         message: String::from("Page not found"),
//     })
// }

// 错误转换 From 特征
// std::convert::From 特征
// pub trait From<T>: Sized {
//   fn from(_: T) -> Self;
// }

// #[derive(Debug)]
// struct AppError {
//     kind: String,    // 错误类型
//     message: String, // 错误信息
// }

// // 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
// // 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
// impl From<io::Error> for AppError {
//     fn from(error: io::Error) -> Self {
//         AppError {
//             kind: String::from("io"),
//             message: error.to_string(),
//         }
//     }
// }

// 上面的例子只有一个标准库错误，再来看看多个不同的错误转换成 AppError 的实现
#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}

// 归一化不同的错误类型
// 如果你要在一个函数中返回不同的错误呢
// fn test_normalize_error() -> Result<(), std::io::Error> {
//     let html = render()?;
//     println!("{}", html);
//     Ok(())
// }

// fn render() -> Result<String, std::io::Error> {
//     // 为了满足 render 函数的签名，我们就需要将 env::VarError 和 io::Error 归一化为同一种错误类型
//     // 要实现这个目的有三种方式
//     // 1. 使用特征对象 Box<dyn Error>
//     // 2. 自定义错误类型
//     // 3. 使用 thiserror

//     // 报错，原因在于 render 函数中的两个 ? 返回的实际上是不同的错误
//     // env::var() 返回的是 std::env::VarError
//     let file = std::env::var("MARKDOWN")?;
//     // read_to_string 返回的是 std::io::Error
//     let source = read_to_string(file)?;
//     Ok(source)
// }

// TODO: 还没认真看，看不进去
// Box<dyn Error>
fn render() -> Result<String, Box<dyn Error>> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

fn test_box_error() -> Result<(), Box<dyn Error>> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}
