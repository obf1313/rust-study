use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
    let x = plus_or_minus(5);
    println!("The value of x is: {}", x);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

// report 函数会隐式返回一个 ()
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

// 下面的函数显式的返回了 ()
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

// 经常在错误提示中看到该 () 的身影出没
// fn add(x: u32, y: u32) -> u32 {
//     x + y;
// }

// 永不返回的发散函数 !
fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
        // ...
    }
}
