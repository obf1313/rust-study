fn main() {
    println!("Hello, world!");
    {
        // s 在这里无效，它尚未声明
        let s = "hello"; // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，s不再有效

    let s = String::from("hello");

    // Rust 为我们提供动态字符串类型: String, 该类型被分配到堆上，因此可以动态伸缩，也就能存储在编译时大小未知的文本。
    // :: 是一种调用操作符，这里表示调用 String 模块中的 from 方法，由于 String 类型存储在堆上，因此它是动态的，你可以这样修改：
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`

    // 因此这两个值都是通过自动拷贝的方式来赋值的，都被存在栈中，完全无需在堆上分配内存。
    let x = 5;
    let y = x;

    // 当 s1 被赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了
    let s1 = String::from("hello");
    let s2 = s1;

    // 由于 Rust 禁止你使用无效的引用
    println!("{}, world!", s1);

    // x 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}

// 深拷贝
fn deep_copy() {
    // Rust永远也不会自动创建数据的 “深拷贝”
    // 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

// 浅拷贝
fn shallow_copy() {
    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

// 函数参数
fn func_params() {
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...

    // ... 所以到这里不再有效
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，

    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn func_return() {
    let s1 = gives_ownership(); // gives_ownership 将返回值

    // 移给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到

    // takes_and_gives_back 中,
    // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}
