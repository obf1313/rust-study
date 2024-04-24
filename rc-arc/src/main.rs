use std::{rc::Rc, sync::Arc, thread};

fn main() {
    println!("Hello, world!");
    test_rc();
    test_rc_count();
}
// Rc 和 Arc，前者适用于单线程，后者适用于多线程

// Rc<T>
// 引用计数(reference counting)，顾名思义，通过记录一个数据被引用的次数来确定该数据是否正在被使用。
// 当引用次数归零时，就代表该数据不再被使用，因此可以被清理释放。
// 当我们希望在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，就可以使用 Rc 成为数据值的所有者
fn test_rc() {
    // 创建了一个新的 Rc<String> 智能指针并赋给变量 a，该指针指向底层的字符串数据。
    // 智能指针 Rc<T> 在创建时，还会将引用计数加 1，此时获取引用计数的关联函数 Rc::strong_count 返回的值将是 1。
    let a = Rc::new(String::from("hello, world"));
    // 使用 Rc::clone 克隆了一份智能指针 Rc<String>，并将该智能指针的引用计数增加到 2。
    // 这里的 clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据，因此 a 和 b 是共享了底层的字符串 s，这种复制效率是非常高的。
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
}

fn test_rc_count() {
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// 不可变引用

struct Owner {
    name: String,
    // ...其它字段
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // ...其它字段
}

fn test_gadget() {
    // 创建一个基于引用计数的 `Owner`.
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
    });

    // 创建两个不同的工具，它们属于同一个主人
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    // 释放掉第一个 `Rc<Owner>`
    drop(gadget_owner);

    // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
    // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
    // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
    // 引用指向底层的 owner 数据，引用计数尚未清零
    // 因此 owner 数据依然可以被使用
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
    // 数据也被清理释放
}

// Rc 简单总结
// 1. Rc/Arc 是不可变引用，你无法修改它指向的值，只能进行读取，如果要修改，需要配合后面章节的内部可变性 RefCell 或互斥锁 Mutex
// 2. 一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
// 3. Rc 只能用于同一线程内部，想要用于线程之间的对象共享，你需要使用 Arc
// 4. Rc<T> 是一个智能指针，实现了 Deref 特征，因此你无需先解开 Rc 指针，再使用里面的 T，而是可以直接使用 T，例如上例中的 gadget1.owner.name

// 多线程无力的 Rc<T>
fn test_thread_rc() {
    let s = Rc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Rc::clone(&s);
        // 表面原因是 Rc<T> 不能在线程间安全的传递，实际上是因为它没有实现 Send 特征，而该特征是恰恰是多线程间传递数据的关键
        // 由于 Rc<T> 需要管理引用计数，但是该计数器并没有使用任何并发原语，因此无法实现原子化的计数操作，最终会导致计数错误
        // let handle = thread::spawn(move || println!("{}", s));
    }
}

// Arc
// Arc 是 Atomic Rc 的缩写，顾名思义：原子化的 Rc<T> 智能指针。
// Arc 和 Rc 并没有定义在同一个模块，前者通过 use std::sync::Arc 来引入，后者通过 use std::rc::Rc
// 这两者都是只读的，如果想要实现内部数据可修改，必须配合内部可变性 RefCell 或者互斥锁 Mutex 来一起使用。
fn test_arc() {
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || println!("{}", s));
    }
}
