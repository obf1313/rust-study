// Rust 中的资源回收
fn main() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    // 1
    println!("Running!");
    // 输出
    // Running!
    // Dropping Foo!
    // Dropping HasTwoDrops!
    // Dropping HasDrop1!
    // Dropping HasDrop2!
}
struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        // 4
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        // 5
        println!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        // 3
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        // 2
        println!("Dropping Foo!")
    }
}

// Drop 的顺序
// 变量级别，按照逆序的方式，_x 在 _foo 之前创建，因此 _x 在 _foo 之后被 drop
// 结构体内部，按照顺序的方式，结构体 _x 中的字段按照定义中的顺序依次 drop

// 没有实现 Drop 的结构体
// Rust 自动为几乎所有类型都实现了 Drop 特征，因此就算你不手动为结构体实现 Drop，它依然会调用默认实现的 drop 函数，同时再调用每个字段的 drop 方法

// 手动回收#[derive(Debug)]
// struct Foo;

// impl Drop for Foo {
//     fn drop(&mut self) {
//         println!("Dropping Foo!")
//     }
// }

// fn test_handle_drop() {
//     let foo = Foo;
//     // 不允许显式地调用析构函数（这是一个用来清理实例的通用编程概念）
//     // foo.drop();
//     // println!("Running!:{:?}", foo);
//     drop(foo);
// }

// Drop 使用场景
// 回收内存资源
// 执行一些收尾工作
// 文件描述符、网络 socket 等，当这些值超出作用域不再使用时，就需要进行关闭以释放相关的资源
// 在这些情况下，就需要使用者自己来解决 Drop 的问题

// 互斥的 Copy 和 Drop
// 我们无法为一个类型同时实现 Copy 和 Drop 特征。
// 因为实现了 Copy 的特征会被编译器隐式的复制，因此非常难以预测析构函数执行的时间和频率。
// 报错
// #[derive(Copy)]
// struct Foo;

// impl Drop for Foo {
//     fn drop(&mut self) {
//         println!("Dropping Foo!")
//     }
// }
