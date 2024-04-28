use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
#![feature(once_cell)]
use std::{lazy::SyncOnceCell, thread};

// 全局变量的生命周期肯定是'static，但是不代表它需要用static来声明
// 编译期初始化
// 静态常量
// const MAX_ID: usize = usize::MAX / 2;
fn main() {
    println!("用户ID允许的最大值是{}", MAX_ID);
    // test_static_var();
}
// 常量与普通变量的区别
// 关键字是const而不是let
// 定义常量必须指明类型（如 i32）不能省略
// 定义常量时变量的命名规则一般是全部大写
// 常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。编译时编译器会尽可能将其内联到代码中，所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址
// 常量的赋值只能是常量表达式/数学表达式，也就是说必须是在编译期就能计算出的值，如果需要在运行时才能得出结果的值比如函数，则不能赋值给常量表达式
// 对于变量出现重复的定义(绑定)会发生变量遮盖，后面定义的变量会遮住前面定义的变量，常量则不允许出现重复的定义

// 静态变量
// 静态变量允许声明一个全局的变量，常用于全局数据统计，例如我们希望用一个变量来统计程序当前的总请求数：
// static mut REQUEST_RECV: usize = 0;
// fn test_static_var() {
//     // Rust 要求必须使用unsafe语句块才能访问和修改static变量，因为这种使用方式往往并不安全
//     unsafe {
//         REQUEST_RECV += 1;
//         assert_eq!(REQUEST_RECV, 1);
//     }
// }

// 静态变量和常量的区别
// 1. 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
// 2. 存储在静态变量中的值必须要实现 Sync trait

// 原子类型
static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
fn test_atomic() {
    for _ in 0..100 {
        REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    }

    println!("当前用户请求数{:?}", REQUEST_RECV);
}
// 全局 ID 生成器
struct Factory {
    factory_id: usize,
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

fn generate_id() -> usize {
    // 检查两次溢出，否则直接加一可能导致溢出
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

// 运行期初始化
// 以上的静态初始化有一个致命的问题：无法用函数进行静态初始化，例如你如果想声明一个全局的Mutex锁：
static NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));

fn test_runtime_init() {
    let v = NAMES.lock().unwrap();
    println!("{}", v);
}
// 使用lazy_static在每次访问静态变量时，会有轻微的性能损失，因为其内部实现用了一个底层的并发原语std::sync::Once，在每次访问该变量时，程序都会执行一次原子指令用于确认静态变量的初始化是否完成。
// lazy_static宏，匹配的是static ref，所以定义的静态变量都是不可变引用
lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}

fn test_lazy_static() {
    let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v);
}

// 一个全局的动态配置，它在程序开始后，才加载数据进行初始化，最终可以让各个线程直接访问使用
// 使用lazy_static实现全局缓存的例子:
lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}
fn test_lazy_static2() {
    // 首次访问`HASHMAP`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}

// Box::leak
#[derive(Debug)]
struct Config {
    a: String,
    b: String
}
static mut CONFIG: Option<&mut Config> = None;
fn test_box_leak() {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // 将`c`从内存中泄漏，变成`'static`生命周期
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);
    }
}
// 从函数中返回全局变量
fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}


fn test_box_leak1() {
    unsafe {
        CONFIG = init();
        println!("{:?}", CONFIG)
    }
}

// 标准库中的 OnceCell
// 在 Rust 标准库中提供了实验性的 lazy::OnceCell 和 lazy::SyncOnceCell (在 Rust 1.70.0版本及以上的标准库中
// 替换为稳定的 cell::OnceCell 和 sync::OnceLock )两种 Cell ，前者用于单线程，后者用于多线程，它们用来存储堆上的信息，并且具有最 多只能赋值一次的特性。 

// 实现一个多线程的日志组件 Logger：
// 低于Rust 1.70版本中， OnceCell 和 SyncOnceCell 的API为实验性的 ，
// 需启用特性 `#![feature(once_cell)]`。

// Rust 1.70版本以上,
// use std::{sync::OnceLock, thread};

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

// 低于Rust 1.70版本
static LOGGER: SyncOnceCell<Logger> = SyncOnceCell::new();

// Rust 1.70版本以上
// static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    // 声明了一个 global() 关联函数
    fn global() -> &'static Logger {
        // 获取或初始化 Logger
        LOGGER.get_or_init(|| {
            println!("Logger is being created..."); // 初始化打印
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message)
    }
}
// Logger is being created...
// some message
// other message
// thread message

// 总结
// 全局变量可以分为两种
// 1. 编译期初始化的全局变量，const创建常量，static创建静态变量，Atomic创建原子类型
// 2. 运行期初始化的全局变量，lazy_static用于懒初始化，Box::leak利用内存泄漏将一个变量的生命周期变为'static