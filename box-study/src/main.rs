fn main() {
    println!("Hello, world!");
    // 当被从 foo 函数转移给 main 中的 b 变量时，栈上的智能指针被复制一份赋予给 b
    let b = foo("world");
    println!("{}", b);

    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
    for e in elems {
        e.draw()
    }
}
// Rust 中的堆栈
// 在 Rust 中，main 线程的栈大小是 8MB，普通线程是 2MB
// 堆内存通常只受物理内存限制
fn foo(x: &str) -> String {
    // a 是 String 类型，它其实是一个智能指针结构体，该智能指针存储在函数栈中，指向堆上的字符串数据
    let a = "Hello, ".to_string() + x;
    a
}

// Box 的使用场景
// Box 是简单的封装，除了将值存储在堆上外，并没有其它性能上的损耗
// Box 相比其它智能指针，功能较为单一，可以在以下场景中使用它
// 1. 特意的将数据分配在堆上
// 2. 数据较大时，又不想在转移所有权时进行数据拷贝
// 3. 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
// 4. 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

// 使用 Box<T> 将数据存储在堆上
fn test_box() {
    let a = Box::new(3);
    // println! 可以正常打印出 a 的值，是因为它隐式地调用了 Deref 对智能指针 a 进行了解引用
    println!("a = {}", a); // a = 3

    // 报错，是因为在表达式中，我们无法自动隐式地执行 Deref 解引用操作，你需要使用 * 操作符 let b = *a + 1，来显式的进行解引用
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
    // a 持有的智能指针将在作用域结束时，被释放掉，这是因为 Box<T> 实现了 Drop 特征
}

// 避免栈上数据的拷贝
fn test_stack_copy() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

// 将动态大小类型变为 Sized 固定大小类型
// Rust 需要在编译时知道类型占用多少空间，如果一种类型在编译时无法知道具体的大小，那么被称为动态大小类型 DST。
// 其中一种无法在编译时知道大小的类型是递归类型：在类型定义中又使用到了自身，或者说该类型的值的一部分可以是相同类型的其它值，这种值的嵌套理论上可以无限进行下去，所以 Rust 不知道递归类型需要多少空间
enum List {
    // 只需要将 List 存储到堆上，然后使用一个智能指针指向它，即可完成从 DST 到 Sized 类型(固定大小类型)的华丽转变
    Cons(i32, Box<List>),
    Nil,
}

// 特征对象
trait Draw {
    fn draw(&self);
}
struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}
struct Select {
    id: u32,
}
impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}

// Box 内存布局
// https://course.rs/advance/smart-pointer/box.html
fn test_box_memory() {
    let arr = vec![Box::new(1), Box::new(2)];
    // 使用 & 借用数组中的元素，否则会报所有权错误
    let (first, second) = (&arr[0], &arr[1]);
    // 我们从数组中取出某个元素时，取到的是对应的智能指针 Box，需要对该智能指针进行解引用
    // 表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，第一次将 &Box<i32> 类型转成 Box<i32>，第二次将 Box<i32> 转成 i32
    let sum = **first + **second;
}

// Box::leak
// 它可以消费掉 Box 并且强制目标值从内存中泄漏
fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");
    // Box::leak 我们不仅返回了一个 &str 字符串切片，它还是 'static 生命周期的
    Box::leak(s.into_boxed_str())
    // 使用场景
    // 你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，那么就可以使用 Box::leak
}

fn test_box_leak() {
    let s = gen_static_str();
    println!("{}", s);
}

// Box 背后是调用 jemalloc 来做内存管理，所以堆上的空间无需我们的手动管理。
