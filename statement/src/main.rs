fn main() {
    let y = {
        let x = 3;
        // 注意 x + 1 不能以分号结尾，否则就会从表达式变成语句， 表达式不能包含分号
        x + 1
    };
    // 总之，能返回值，它就是表达式
    // 调用一个函数是表达式，因为会返回一个值，调用宏也是表达式
    println!("The value of y is: {}", y);
    // 表达式如果不返回任何值，会隐式地返回一个 () 。
    assert_eq!(ret_unit_type(), ());
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    // 语句会执行一些操作但是不会返回一个值
    let x = x + 1; // 语句
    let y = y + 5; // 语句
                   // 表达式会在求值后返回一个值
    x + y // 表达式
}

fn statement() {
    // 由于 let 是语句，因此不能将 let 语句赋值给其它值
    // let b = (let a = 8);
}

fn ret_unit_type() {
    let x = 1;
    let y = if x % 2 == 1 { "odd" } else { "even" };
}
