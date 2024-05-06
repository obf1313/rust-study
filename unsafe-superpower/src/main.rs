fn main() {
    println!("Hello, world!");
    create_raw_pointer();
}

// 解引用裸指针
// 裸指针(raw pointer，又称原生指针) 在功能上跟引用类似，同时它也需要显式地注明可变性。
// 但是又和引用有所不同，裸指针长这样: *const T 和 *mut T，它们分别代表了不可变和可变。

// 三种类似指针的概念
// 引用、智能指针和裸指针
// 与前两者不同，裸指针：
// 1. 可以绕过 Rust 的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
// 2. 并不能保证指向合法的内存
// 3. 可以是 null
// 4. 没有实现任何自动的回收 (drop)

fn create_raw_pointer() {
    // 基于引用创建裸指针
    let mut num = 5;
    // as 可以用于强制类型转换
    // 我们将引用 &num / &mut num 强转为相应的裸指针 *const i32 / *mut i32
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为
    unsafe {
        println!("r1 is: {}", *r1);
    }

    // 基于内存地址创建裸指针
    let address = 0x012345usize;
    let r = address as *const i32;

    // 如果真的要使用内存地址，也是类似下面的用法，先取地址，再使用，而不是凭空捏造一个地址：
}
