use std::hint;
use std::ops::Sub;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;

// 线程同步：Atomic 原子类型与内存顺序
fn main() {
    println!("Hello, world!");
    test_atomic();
}

// 从 Rust1.34 版本后，就正式支持原子类型。
// 原子指的是一系列不可被 CPU 上下文交换的机器指令，这些指令组合在一起就形成了原子操作。
// 在多核 CPU 下，当某个 CPU 核心开始运行原子操作时，会先暂停其它 CPU 内核对内存的操作，以保证原子操作不会被其它 CPU 内核所干扰。
// 由于原子操作是通过指令提供的支持，因此它的性能相比锁和消息传递会好很多。
// 相比较于锁而言，原子类型不需要开发者处理加锁和释放锁的问题，同时支持修改，读取等操作，还具备较高的并发性能，几乎所有的语言都支持原子类型。
// 可以看出原子类型是无锁类型，但是无锁不代表无需等待，因为原子类型内部使用了CAS循环，当大量的冲突发生时，该等待还是得等待！但是总归比锁要好。
// CAS 全称是 Compare and swap, 它通过一条指令读取指定的内存地址，然后判断其中的值是否等于给定的前置值，如果相等，则将其修改为新的值

// 使用 Atomic 作为全局变量
const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        // 每个线程都在累加，累加 10000000 次
        for _ in 0..n {
            // Ordering::Relaxed, 看上去很像是排序作用，但是我们并没有做排序操作啊？实际上它用于控制原子操作使用的内存顺序。
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn test_atomic() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    // 10个线程
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    // Ordering::Relaxed, 看上去很像是排序作用，但是我们并没有做排序操作啊？实际上它用于控制原子操作使用的内存顺序。
    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    // 时间
    println!("{:?}", Instant::now().sub(s));
}

// 可以通过Mutex来实现，但是后者的强大功能是建立在额外的性能损耗基础上的，因此性能会逊色不少
// Atomic实现会比Mutex快41%，实际上在复杂场景下还能更快(甚至达到 4 倍的性能差距)！

// 和Mutex一样，Atomic的值具有内部可变性，你无需将其声明为mut
struct Counter {
    count: u64,
}
fn test_mutex_atomic() {
    let n = Mutex::new(Counter { count: 0 });
    n.lock().unwrap().count += 1;
    let n = AtomicU64::new(0);
    // Ordering::Relaxed, 看上去很像是排序作用，但是我们并没有做排序操作啊？实际上它用于控制原子操作使用的内存顺序。
    n.fetch_add(0, Ordering::Relaxed);
}

// 内存顺序
// 内存顺序是指 CPU 在访问内存时的顺序，该顺序可能受以下因素的影响
// 代码中的先后顺序
// 编译器优化导致在编译阶段发生改变(内存重排序 reordering)
// 运行阶段因 CPU 的缓存机制导致顺序被打乱

// 编译器优化导致内存顺序的改变
// static mut X: u64 = 0;
// static mut Y: u64 = 1;

// fn test_memory_change() {
//     ...     // A
//     unsafe {
//         ... // B
//         X = 1;
//         ... // C
//         Y = 3;
//         ... // D
//         X = 2;
//         ... // E
//     }
// }
// 假如在C和D代码片段中，根本没有用到X = 1，那么编译器很可能会将X = 1和X = 2进行合并:
// ...     // A
// unsafe {
//     ... // B
//     X = 2;
//     ... // C
//     Y = 3;
//     ... // D
//     ... // E
// }
// 若代码A中创建了一个新的线程用于读取全局静态变量X，则该线程将无法读取到X = 1的结果，因为在编译阶段就已经被优化掉。

// CPU 缓存导致的内存顺序的改变
// 假设之前的X = 1没有被优化掉，并且在代码片段A中有一个新的线程:
// initial state: X = 0, Y = 1

// THREAD Main     THREAD A
// X = 1;          if X == 1 {
// Y = 3;              Y *= 2;
// X = 2;          }
// 我们来讨论下以上线程状态，Y最终的可能值(可能性依次降低):
// Y = 3: 线程Main运行完后才运行线程A，或者线程A运行完后再运行线程Main
// Y = 6: 线程Main的Y = 3运行完，但X = 2还没被运行， 此时线程 A 开始运行Y *= 2, 最后才运行Main线程的X = 2
// Y = 2: 线程Main正在运行Y = 3还没结束，此时线程A正在运行Y *= 2, 因此Y取到了值 1，然后Main的线程将Y设置为 3， 紧接着就被线程A的Y = 2所覆盖
// Y = 2: 上面的还只是一般的数据竞争，这里虽然产生了相同的结果2，但是背后的原理大相径庭: 线程Main运行完Y = 3，但是 CPU 缓存中的Y = 3还没有被同步到其它 CPU 缓存中，此时线程A中的Y *= 2就开始读取Y，结果读到了值1，最终计算出结果2

// 甚至更改成:
// initial state: X = 0, Y = 1

// THREAD Main     THREAD A
// X = 1;          if X == 2 {
// Y = 3;              Y *= 2;
// X = 2;          }
// 还是可能出现Y = 2，因为Main线程中的X和Y被同步到其它 CPU 缓存中的顺序未必一致。

// 限定内存顺序的 5 个规则
// Rust 提供了Ordering::Relaxed用于限定内存顺序,事实上，该枚举有 5 个成员
// Relaxed， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序
// Release 释放，设定内存屏障(Memory barrier)，保证它之前的操作永远在它之前，但是它后面的操作可能被重排到它前面
// Acquire 获取, 设定内存屏障，保证在它之后的访问永远在它之后，但是它之前的操作却有可能被重排到它后面，往往和Release在不同线程中联合使用
// AcqRel, 是 Acquire 和 Release 的结合，同时拥有它们俩提供的保证。比如你要对一个 atomic 自增 1，同时希望该操作之前和之后的读取或写入操作不会被重新排序
// SeqCst 顺序一致性， SeqCst就像是AcqRel的加强版，它不管原子操作是属于读取还是写入的操作，只要某个线程有用到SeqCst的原子操作，线程中该SeqCst操作前的数据操作绝对不会被重新排在该SeqCst操作之后，且该SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前。

// 内存屏障的例子
// 以Release和Acquire为例，使用它们构筑出一对内存屏障，防止编译器和 CPU 将屏障前(Release)和屏障后(Acquire)中的数据操作重新排在屏障围成的范围之外

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100; // A
        }
        // Release用于写入
        READY.store(true, Ordering::Release); // B: 内存屏障 ↑
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        // Acquire用于读取
        while !READY.load(Ordering::Acquire) {} // C: 内存屏障 ↓

        assert_eq!(100, unsafe { DATA }); // D
    })
}

// 测试内存屏障
fn test_memory_barrier() {
    loop {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
// 但是由于有些原子操作同时拥有读取和写入的功能，此时就需要使用AcqRel来设置内存顺序了。
// 在内存屏障中被写入的数据，都可以被其它线程读取到，不会有 CPU 缓存的问题。

// 内存顺序的选择
// 不知道怎么选择时，优先使用SeqCst，虽然会稍微减慢速度，但是慢一点也比出现错误好
// 多线程只计数fetch_add而不使用该值触发其他逻辑分支的简单使用场景，可以使用Relaxed
// 参考 Which std::sync::atomic::Ordering to use?

// 多线程中使用 Atomic
// 在多线程环境中要使用Atomic需要配合Arc：
fn test_atomic_multi_thread() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待其它线程释放锁
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}

// Atomic 能替代锁吗
// 那么原子类型既然这么全能，它可以替代锁吗？答案是不行：
// 对于复杂的场景下，锁的使用简单粗暴，不容易有坑
// std::sync::atomic包中仅提供了数值类型的原子操作：AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU16等，而锁可以应用于各种类型
// 在有些情况下，必须使用锁来配合，例如上一章节中使用Mutex配合Condvar

// Atomic 的应用场景
// Atomic虽然对于用户不太常用，但是对于高性能库的开发者、标准库开发者都非常常用，它是并发原语的基石，除此之外，还有一些场景适用
// 无锁(lock free)数据结构
// 全局变量，例如全局自增 ID, 在后续章节会介绍
// 跨线程计数器，例如可以用于统计指标
