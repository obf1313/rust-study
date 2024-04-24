use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

fn main() {
    println!("Hello, world!");
    test_cell();
    test_refcell();
}

// Rust 提供了 Cell 和 RefCell 用于内部可变性，简而言之，可以在拥有不可变引用的同时修改目标数据
// Cell
// Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况
fn test_cell() {
    // "asdf" 是 &str 类型，它实现了 Copy 特征
    let c = Cell::new("asdf");
    // c.get 用来取值，c.set 用来设置新值
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two);
    // 编译器会立刻报错，因为 String 没有实现 Copy 特征：
    // 没报错啊
    // TODO
    let c = Cell::new(String::from("asdf"));
}

// RefCell
// Rust 规则	                     智能指针带来的额外规则
// 一个数据只有一个所有者	           Rc/Arc让一个数据可以拥有多个所有者
// 要么多个不可变借用，要么一个可变借用	RefCell实现编译期可变、不可变引用共存
// 违背规则导致编译错误	               违背规则导致运行时panic
fn test_refcell() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();
    println!("{},{}", s1, s2);
}

// RefCell 简单总结
// 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
// RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
// RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
// 使用 RefCell 时，违背借用规则会导致运行期的 panic

// 选择 Cell 还是 RefCell
// Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
// Cell 不会 panic，而 RefCell 会

// 内部可变性
fn test_mut() {
    let x = 5;
    // 上面的代码会报错，因为我们不能对一个不可变的值进行可变借用，这会破坏 Rust 的安全性保证
    // 相反，你可以对一个可变值进行不可变借用。
    // 原因是：当值不可变时，可能会有多个不可变的引用指向它，此时若将其中一个修改为可变的，会造成可变引用与不可变引用共存的情况；
    // 而当值可变时，最多只会有一个可变引用指向它，将其修改为不可变，那么最终依然是只有一个不可变的引用指向它。
    // let y = &mut x;
}

// 定义在外部库中的特征
pub trait Messenger {
    // 外部库中定义了一个消息发送器特征 Messenger，它只有一个发送消息的功能：fn send(&self, msg: String)，因为发送消息不需要修改自身，因此原作者在定义时，使用了 &self 的不可变借用，这个无可厚非。
    fn send(&self, msg: String);
}

// --------------------------
// 我们的代码中的数据结构和实现
// struct MsgQueue {
//     msg_cache: Vec<String>,
// }

// impl Messenger for MsgQueue {
//     fn send(&self, msg: String) {
//         self.msg_cache.push(msg)
//     }
// }

pub struct MsgQueue {
    // 通过包裹一层 RefCell，成功的让 &self 中的 msg_cache 成为一个可变值，然后实现对其的修改
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn test_msgQueue() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello, world".to_string());
}

// Rc + RefCell 组合使用
fn test_rc_refcell() {
    // 我们使用 RefCell<String> 包裹一个字符串，同时通过 Rc 创建了它的三个所有者：s、s1和s2
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));
    let s1 = s.clone();
    let s2 = s.clone();
    // 所有者 s2 对字符串内容进行了修改
    s2.borrow_mut().push_str(", oh yeah!");
    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
    // 由于 Rc 的所有者们共享同一个底层的数据，因此当一个所有者修改了数据时，会导致全部所有者持有的数据都发生了变化。
}

// 通过 Cell::from_mut 解决借用冲突
// 在 Rust 1.37 版本中新增了两个非常实用的方法：
// Cell::from_mut，该方法将 &mut T 转为 &Cell<T>
// Cell::as_slice_of_cells，该方法将 &Cell<[T]> 转为 &[Cell<T>]
fn is_even(i: i32) -> bool {
    i % 2 == 0
}

// fn retain_even(nums: &mut Vec<i32>) {
//     let mut i = 0;
//     for num in nums.iter().filter(|&num| is_even(*num)) {
//         // 报错
//         // 报错是因为同时借用了不可变与可变引用
//         nums[i] = *num;
//         i += 1;
//     }
//     nums.truncate(i);
// }

fn retain_even(nums: &mut Vec<i32>) {
    // 这两个方法可以很方便的帮我们把 &mut [T] 类型转换成 &[Cell<T>] 类型
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }
    nums.truncate(i);
}
