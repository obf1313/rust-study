use lazy_static::lazy_static;
use std::{
    rc::Rc,
    sync::{Arc, Condvar, Mutex, MutexGuard, RwLock},
    thread::{self, sleep, spawn},
    time::Duration,
};

// 线程同步：锁、Condvar 和信号量
fn main() {
    println!("Hello, world!");
}
// 该如何选择
// 共享内存可以说是同步的灵魂，因为消息传递的底层实际上也是通过共享内存来实现，两者的区别如下
// 1. 共享内存相对消息传递能节省多次内存拷贝的成本
// 2. 共享内存的实现简洁的多
// 3. 共享内存的锁竞争更多

// 消息传递适用的场景很多，我们下面列出了几个主要的使用场景:
// 1. 需要可靠和简单的(简单不等于简洁)实现时
// 2. 需要模拟现实世界，例如用消息去通知某个目标执行相应的操作时
// 3. 需要一个任务处理流水线(管道)时，等等

// 而使用共享内存(并发原语)的场景往往就比较简单粗暴：
// 需要简洁的实现以及更高的性能时。

// 总之，消息传递类似一个单所有权的系统：一个值同时只能有一个所有者，如果另一个线程需要该值的所有权，需要将所有权通过消息传递进行转移。而共享内存类似于一个多所有权的系统：多个线程可以同时访问同一个值。

// 互斥锁 Mutex
// Mutex让多个线程并发的访问同一个值变成了排队访问：同一时间，只允许一个线程A访问该值，其它线程需要等待A访问完成后才能继续。

// 单线程中使用 Mutex
fn test_mutex_single_thread() {
    // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
    let m = Mutex::new(5);
    {
        // 获取锁，然后deref为`m`的引用
        // lock返回的是Result
        // 和Box类似，数据被Mutex所拥有，要访问内部的数据，需要使用方法m.lock()向m申请一个锁
        // 该方法会阻塞当前线程，直到获取到锁，因此当多个线程同时访问该数据时，只有一个线程能获取到锁
        // 其它线程只能阻塞着等待，这样就保证了数据能被安全的修改！
        // m.lock()方法也有可能报错
        // 例如当前正在持有锁的线程panic了。在这种情况下，其它线程不可能再获得锁，因此lock方法会返回一个错误。
        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁自动被drop
    }
    println!("m = {:?}", m);
}
fn test_dead_lock() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
    // drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    // drop(num1); // 手动 drop num1 ，观察打印结果的不同

    println!("m = {:?}", m);
}

// 多线程中使用 Mutex
fn test_mutex_multi_thread() {
    // 通过`Rc`实现`Mutex`的多所有权
    // 多线程安全的 Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有子线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终的计数结果
    println!("Result: {}", *counter.lock().unwrap());
}

// 内部可变性
// Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutex<T>用于多线程内部可变性。

// 需要小心使用的 Mutex
// 1. 在使用数据前必须先获取锁
// 2. 在数据使用完成后，必须及时的释放锁，比如文章开头的例子，使用内部语句块的目的就是为了及时的释放锁
// 创建死锁(deadlock)：当一个操作试图锁住两个资源，然后两个线程各自获取其中一个锁，并试图获取另一个锁时，就会造成死锁。

// 死锁
// 单线程死锁
fn test_deadlock_single_thread() {
    let data = Mutex::new(0);
    let d1 = data.lock();
    // 只要你在另一个锁还未被释放时去申请新的锁，就会触发
    let d2 = data.lock();
} // d1锁在此处释放

// 多线程死锁
// 当我们拥有两个锁，且两个线程各自使用了其中一个锁，然后试图去访问另一个锁时，就可能发生死锁

lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

fn test_deadlock_multi_thread() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    // 线程 1 锁住了MUTEX1并且线程2锁住了MUTEX2
                    // 而与此同时，线程 1 需要等待线程 2 释放MUTEX2后才能释放MUTEX1
                    // 两个线程都无法释放对方需要的锁，最终死锁。
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    // 然后线程 1 试图去访问MUTEX2
                    let guard = MUTEX2.lock().unwrap();
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                    // 同时线程2试图去访问MUTEX1，就会死锁
                    // 因为线程 2 需要等待线程 1 释放MUTEX1后，才会释放MUTEX2
                    let _guard = MUTEX1.lock().unwrap();
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}

// try_lock
// 与lock方法不同，try_lock会尝试去获取一次锁，如果无法获取会返回一个错误，因此不会发生阻塞
fn test_try_lock() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.try_lock();
                    println!("线程 {} 获取 MUTEX2 锁的结果: {:?}", i_thread, guard);
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                    sleep(Duration::from_millis(10));
                    let guard = MUTEX1.try_lock();
                    println!("线程 {} 获取 MUTEX1 锁的结果: {:?}", i_thread, guard);
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}
// 当try_lock失败时，会报出一个错误:Err("WouldBlock")
// 接着线程中的剩余代码会继续执行，不会被阻塞。

// 读写锁 RwLock
// Mutex会对每次读写都进行加锁，但某些时候，我们需要大量的并发读，Mutex就无法满足需求了，此时就可以使用RwLock:
// RwLock在使用上和Mutex区别不大，只有在多个读的情况下不阻塞程序，其他如读写、写读、写写情况下均会对后获取锁的操作进行阻塞。
fn test_rwLock() {
    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        // 我们也可以使用try_write和try_read来尝试进行一次写/读，若失败则返回错误:
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    } // 写锁在此处被drop
}
// 简单总结下RwLock:
// 1. 同时允许多个读，但最多只能有一个写
// 2. 读和写不能同时存在
// 3. 读可以使用read、try_read，写write、try_write, 在实际项目中，try_xxx会安全的多

// Mutex 还是 RwLock
// 首先简单性上Mutex完胜，因为使用RwLock你得操心几个问题：
// 1. 读和写不能同时发生，如果使用try_xxx解决，就必须做大量的错误处理和失败重试机制
// 2. 当读多写少时，写操作可能会因为一直无法获得锁导致连续多次失败(writer starvation)
// 3. RwLock 其实是操作系统提供的，实现原理要比Mutex复杂的多，因此单就锁的性能而言，比不上原生实现的Mutex

// 再来简单总结下两者的使用场景：
// 1. 追求高并发读取时，使用RwLock，因为Mutex一次只允许一个线程去读取
// 2. 如果要保证写操作的成功性，使用Mutex
// 3. 不知道哪个合适，统一使用Mutex
// RwLock虽然看上去貌似提供了高并发读取的能力，但这个不能说明它的性能比Mutex高，事实上Mutex性能要好不少，后者唯一的问题也仅仅在于不能并发读取。
// 一个常见的、错误的使用RwLock的场景就是使用HashMap进行简单读写，因为HashMap的读和写都非常快，RwLock的复杂实现和相对低的性能反而会导致整体性能的降低，因此一般来说更适合使用Mutex。
// 总之，如果你要使用RwLock要确保满足以下两个条件：并发读，且需要对读到的资源进行"长时间"的操作
// HashMap也许满足了并发读的需求，但是往往并不能满足后者："长时间"的操作。

// 三方库提供的锁实现
// parking_lot, 功能更完善、稳定，社区较为活跃，star 较多，更新较为活跃
// spin, 在多数场景中性能比parking_lot高一点，最近没怎么更新

// 用条件变量(Condvar)控制线程的同步
// Mutex用于解决资源安全访问的问题，但是我们还需要一个手段来解决资源访问顺序的问题。
// 而 Rust 考虑到了这一点，为我们提供了条件变量(Condition Variables)
// 它经常和Mutex一起使用，可以让线程挂起，直到某个条件发生后再继续执行
fn test_condvar() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                lock = ccond.wait(lock).unwrap();
            }

            *lock = false;

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}

// 信号量 Semaphore
// 另一个重要的概念就是信号量，使用它可以让我们精准的控制当前正在运行的任务最大数量
// 我们推荐使用tokio中提供的Semaphore实现: tokio::sync::Semaphore。
// TODO: 库已经换了写法？
// async fn test_semaphore() {
//     // 创建了一个容量为 3 的信号量，当正在执行的任务超过 3 时
//     // 剩下的任务需要等待正在执行任务完成并减少信号量后到 3 以内时，才能继续执行。
//     let semaphore = Arc::new(Semaphore::new(3));
//     let mut join_handles = Vec::new();

//     for _ in 0..5 {
//         let permit = semaphore.clone().acquire_owned().await.unwrap();
//         join_handles.push(tokio::spawn(async move {
//             //
//             // 在这里执行任务...
//             //
//             drop(permit);
//         }));
//     }

//     for handle in join_handles {
//         handle.await.unwrap();
//     }
// }
