// 闭包
use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");
    test_closure();
}

// 闭包是一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值
fn test_closure() {
    let x = 1;
    // sum 非常符合闭包的定义：可以赋值给变量，允许捕获调用者作用域中的值。
    let sum = |y| x + y;
    assert_eq!(3, sum(2));
    // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;
    // 开始健身
    workout(intensity, random_number);

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // 当编译器推导出一种类型后，它就会一直使用该类型
    // let n = example_closure(5);

    // 捕获作用域中的值
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // 对于函数来说，就算你把函数定义在 main 函数体中，它也不能访问 x
    // 这点倒是和 JS 不一样
    // let x = 4;
    // fn equal_to_x(z: i32) -> bool {
    //     // can't capture dynamic environment in a fn item
    //     // use the `|| { ... }` closure form instead
    //     z == x
    // }
    // let y = 4;
    // assert!(equal_to_x(y));
}

// 开始健身，好累，我得发出声音：muuuu...
fn muuuuu(intensity: u32) -> u32 {
    println!("muuuuu...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32) {
    // 闭包
    // |param1, param2,...| {
    //     语句1;
    //     语句2;
    //     返回表达式
    // }
    // 省略参数、返回值类型和花括号对
    let action = || {
        println!("muuuuu....");
        thread::sleep(Duration::from_secs(2));
        // 闭包中最后一行表达式返回的值，就是闭包执行后的返回值，因此 action() 调用返回了 intensity 的值 10
        intensity
    };
    if intensity < 25 {
        println!("今天活力满满，先做 {} 个俯卧撑!", action());
        println!("旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!", action());
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!("昨天练过度了，今天干干有氧，跑步 {} 分钟!", action());
    }
}

// 结构体中的闭包
struct Cacher<T, E>
where
    T: Fn(E) -> E,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    // 约束
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// 三种 Fn 特征
// 1. FnOnce，该类型的闭包会拿走被捕获变量的所有权。Once 顾名思义，说明该闭包只能运行一次
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    // 仅实现 FnOnce 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用
    // func 的类型 F 实现了 Copy 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移
    println!("{}", func(4));
}

// 如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。
fn test_fn_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

// 2. FnMute，它以可变借用的方式捕获了环境中的值，因此可以修改该值：
fn test_fn_mute() {
    let mut s = String::new();
    // 改为 mut update_string
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s);

    let mut s = String::new();
    // 事实上，FnMut只是trait的名字，声明变量为FnMut和要不要mut没啥关系，FnMut是推导出的特征类型
    // mut是rust语言层面的一个修饰符，用于声明一个绑定是可变的。
    let update_string = |str| s.push_str(str);
    // 这段代码中update_string看似被声明为不可变闭包，但是exec(mut f: F)函数接收的又是可变参数
    // 为什么可以正常执行呢？
    // update_string闭包的所有权被移交给了exec函数
    // 这说明update_string没有实现Copy特征
    // 闭包自动实现Copy特征的规则是，只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征。
    exec(update_string);
    println!("{:?}", s);

    let s = String::new();
    // 这里取得的是s的不可变引用，所以是能Copy的。
    let update_string = || println!("{}", s);

    // 拿所有权
    let s = String::new();
    let update_string = move |_| println!("{}", s);
    exec(update_string);
    // exec2(update_string); // 不能再用了

    // 可变引用
    let mut s = String::new();
    let mut update_string = |_| s.push_str("hello");
    exec(update_string);
    // exec1(update_string); // 不能再用了
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

// 3. Fn 特征，它以不可变借用的方式捕获环境中的值
// 让我们把上面的代码中 exec 的 F 泛型参数类型修改为 Fn(&'a str)：
fn test_fn_impl() {
    let mut s = String::new();
    // 不可变借用方式
    let update_string = |str| println!("{},{}", s, str);
    exec1(update_string);
    println!("{:?}", s);
}
fn exec1<'a, F: Fn(&'a str)>(mut f: F) {
    f("hello")
}

// move 和 Fn
// 一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们。
// move 本身强调的就是后者，闭包如何捕获变量
fn test_move_fn() {
    let s = String::new();
    let update_string = move || println!("{}", s);
    exec2(update_string);
}
fn exec2<F: Fn()>(f: F) {
    f()
}

// 三种 Fn 的关系
// 实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下
// 1. 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
// 2. 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
// 3. 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
fn test_all_fn() {
    let s = String::new();
    let update_string = || println!("{}", s);
    exec3(update_string);
    exec4(update_string);
    exec5(update_string);

    let mut s = String::new();
    // let update_string = |str| -> String {
    //     s.push_str(str);
    //     // 此例中，闭包从捕获环境中移出了变量 s 的所有权，因此这个闭包仅自动实现了 FnOnce，未实现 FnMut 和 Fn
    //     // 一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们
    //     s
    // };
    // exec6(update_string);
}
fn exec3<F: FnOnce()>(f: F) {
    f()
}

fn exec4<F: FnMut()>(mut f: F) {
    f()
}

fn exec5<F: Fn()>(f: F) {
    f()
}

fn exec6<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    f("hello");
}

fn test_factory() {
    // 闭包作为函数返回值
    // fn factory() -> impl Fn(i32) -> i32 {
    //     let num = 5;
    //     move |x| x + num
    // }
    // // Rust 要求函数的参数和返回类型，必须有固定的内存大小，例如 i32 就是 4 个字节，引用类型是 8 个字节
    // let f = factory();
    // let answer = f(1);
    // assert_eq!(6, answer);

    // 就算签名一样的闭包，类型也是不同的
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x - num)
        }
    }
}
