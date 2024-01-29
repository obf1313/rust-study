fn main() {
    shadowing();
}

fn variable() {
    // mut 使其成为真正的变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // 你希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头
    let _y = 5;
    let z = 6;
    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    // a = true, 不可变；b = false，可变
    println!("a={:?}, b={:?}", a, b);
    b = true;
    assert_eq!(a, b);
}

struct Struct {
    e: i32,
}

fn value() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    // Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词，另外对数字字面量可插入下划线以提高可读性
    const MAX_POINTS: u32 = 100_000;
}

fn shadowing() {
    // 变量遮蔽
    let x = 5;
    // 在该函数的作用域对之前的 x 进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的 x 进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    // 这种结构是允许的，因为第一个 spaces 变量是一个字符串类型，第二个 spaces 变量是一个全新的变量且和第一个具有相同的变量名，且是一个数值类型。
    // 字符串类型
    let spaces = "    ";
    // usize 数值类型
    let spaces = spaces.len();
    // 但是这样不行，Rust 对类型的要求很严格，不允许将整数类型 usize 赋值给字符串类型。
    // let mut spaces = "   ";
    // spaces = spaces.len();
}

fn infer() {
    // type must be known at this point
    let guess: i32 = "42".parse().expect("Not a number!");
}
