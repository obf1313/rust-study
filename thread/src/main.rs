use std::cell::RefCell;
use std::sync::{Condvar, Mutex, Once};
use std::{
    sync::{Arc, Barrier},
    thread,
    time::Duration,
};

fn main() {
    println!("Hello, world!");
    // test_thread();
    test_not_kill();
}

// 创建线程
fn test_thread() {
    // 使用 thread::spawn 可以创建线程
    // 线程内部的代码使用闭包来执行
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
// main 线程一旦结束，程序就立刻结束，因此需要保持它的存活，直到其它子线程完成自己的任务
// thread::sleep 会让当前线程休眠指定的时间，随后其它线程会被调度运行（上一节并发与并行中有简单介绍过）
// 因此就算你的电脑只有一个 CPU 核心，该程序也会表现的如同多 CPU 核心一般，这就是并发！

// 等待子线程的结束
// 上面的代码你不但可能无法让子线程从 1 顺序打印到 10，而且可能打印的数字会变少，因为主线程会提前结束，导致子线程也随之结束
// 更过分的是，如果当前系统繁忙，甚至该子线程还没被创建，主线程就已经结束了！
// 因此我们需要一个方法，让主线程安全、可靠地等所有子线程完成任务后，再 kill self：
fn test_not_kill() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 通过调用 handle.join，可以让当前线程阻塞，直到它等待的子线程的结束
    // 由于 main 线程会被阻塞，因此它直到子线程结束后才会输出自己的 1..5：在上面代码中
    // 如果你将 handle.join 放置在 main 线程中的 for 循环后面，那就是另外一个结果：两个线程交替输出
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();
}

// 在线程闭包中使用 move
// move 关键字在闭包中的使用可以让该闭包拿走环境中某个值的所有权，同样地，你可以使用 move 来将所有权从一个线程转移到另外一个线程。
fn test_move() {
    let v = vec![1, 2, 3];
    // 线程的启动时间点和结束时间点是不确定的，因此存在一种可能，当主线程执行完， v 被释放掉时，新的线程很可能还没有结束甚至还没有被创建成功，此时新线程对 v 的引用立刻就不再合法
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// 线程是如何结束的
// 线程的代码执行完，线程就会自动结束。但是如果线程中的代码不会执行完呢？那么情况可以分为两种进行讨论
// 线程的任务是一个循环 IO 读取，任务流程类似：IO 阻塞，等待读取新的数据 -> 读到数据，处理完成 -> 继续阻塞等待 ··· -> 收到 socket 关闭的信号 -> 结束线程，在此过程中，绝大部分时间线程都处于阻塞的状态，因此虽然看上去是循环，CPU 占用其实很小，也是网络服务中最最常见的模型
// 线程的任务是一个循环，里面没有任何阻塞，包括休眠这种操作也没有，此时 CPU 很不幸的会被跑满，而且你如果没有设置终止条件，该线程将持续跑满一个 CPU 核心，并且不会被终止，直到 main 线程的结束
fn test_finish() {
    // 创建一个线程A
    let new_thread = thread::spawn(move || {
        // 再创建一个线程B
        thread::spawn(move || loop {
            println!("I am a new thread.");
        })
    });

    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    thread::sleep(Duration::from_millis(100));
}

// 多线程的性能
// 据不精确估算，创建一个线程大概需要 0.24 毫秒，随着线程的变多，这个值会变得更大，因此线程的创建耗时是不可忽略的，只有当真的需要处理一个值得用线程去处理的任务时，才使用线程
// 一些鸡毛蒜皮的任务，就无需创建线程了。

// 创建多少线程合适
// 因为 CPU 的核心数限制，当任务是 CPU 密集型时，就算线程数超过了 CPU 核心数，也并不能帮你获得更好的性能，因为每个线程的任务都可以轻松让 CPU 的某个核心跑满，既然如此，让线程数等于 CPU 核心数是最好的。
// 但是当你的任务大部分时间都处于阻塞状态时，就可以考虑增多线程数量，这样当某个线程处于阻塞状态时，会被切走，进而运行其它的线程，典型就是网络 IO 操作，我们可以为每一个进来的用户连接创建一个线程去处理，该连接绝大部分时间都是处于 IO 读取阻塞状态，因此有限的 CPU 核心完全可以处理成百上千的用户连接线程，但是事实上，对于这种网络 IO 情况，一般都不再使用多线程的方式了，毕竟操作系统的线程数是有限的，意味着并发数也很容易达到上限，而且过多的线程也会导致线程上下文切换的代价过大，使用 async/await 的 M:N 并发模型，就没有这个烦恼。

// 多线程的开销
// 无锁实现(CAS)的 Hashmap 在多线程下的使用
// TODO: 没太看懂
// https://course.rs/advance/concurrency-with-threads/thread.html
// 吞吐并不是线性增长，尤其从 16 核开始，甚至开始肉眼可见的下降
// 1. 虽然是无锁，但是内部是 CAS 实现，大量线程的同时访问，会让 CAS 重试次数大幅增加
// 2. 线程过多时，CPU 缓存的命中率会显著下降，同时多个线程竞争一个 CPU Cache-line 的情况也会经常发生
// 3. 大量读写可能会让内存带宽也成为瓶颈
// 4. 读和写不一样，无锁数据结构的读往往可以很好地线性增长，但是写不行，因为写竞争太大
// 多线程的开销往往是在锁、数据竞争、缓存失效上，这些限制了现代化软件系统随着 CPU 核心的增多性能也线性增加的野心。
fn test_hashmap() {
    // for i in 0..num_threads {
    //     let ht = Arc::clone(&ht);

    //     let handle = thread::spawn(move || {
    //         for j in 0..adds_per_thread {
    //             let key = thread_rng().gen::<u32>();
    //             let value = thread_rng().gen::<u32>();
    //             ht.set_item(key, value);
    //         }
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }
}

// 线程屏障(Barrier)
fn test_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// 线程局部变量(Thread Local Variable)
// 标准库 thread_local
// 使用 thread_local 宏可以初始化线程局部变量，然后在线程内部使用该变量的 with 方法获取变量值
fn test_thread_local_variable() {
    // FOO 即是我们创建的线程局部变量，每个新的线程访问它时，都会使用它的初始值作为开始
    // 各个线程中的 FOO 值彼此互不干扰。注意 FOO 使用 static 声明为生命周期为 'static 的静态变量
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到线程局部变量的FOO的初始值
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    // 等待线程完成
    t.join().unwrap();

    // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

// 在结构体中使用线程局部变量
struct Foo;
impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }
}
fn test_struct_thread_local_variable() {
    Foo::FOO.with(|x| println!("{:?}", x));
}
// 通过引用的方式使用它
// thread_local! {
//     static FOO: RefCell<usize> = RefCell::new(0);
// }
// struct Bar {
//     // TODO: 这是啥
//     foo: &'static LocalKey<RefCell<usize>>,
// }
// impl Bar {
//     fn constructor() -> Self {
//         Self { foo: &FOO }
//     }
// }

// 用条件控制线程的挂起和执行
// 条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起，直到某个条件发生后再继续执行
fn test_condition() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        // 子线程获取到锁，并将其修改为 true，然后调用条件变量的 notify_one 方法来通知主线程继续执行
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    // 首先进入 while 循环，调用 wait 方法挂起等待子线程的通知，并释放了锁 started
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

// 只被调用一次的函数
static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn test_once() {
    let handle1 = thread::spawn(move || {
        // 执行初始化过程一次，并且只执行一次。
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}
// 代码运行的结果取决于哪个线程先调用 INIT.call_once
// 当这个函数返回时，保证一些初始化已经运行并完成，它还保证由执行的闭包所执行的任何内存写入都能被其他线程在这时可靠地观察到。
