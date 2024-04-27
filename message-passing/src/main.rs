use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

fn main() {
    println!("Hello, world!");
    test_try_recv();
    test_async_channel();
}
// 在多线程间有多种方式可以共享、传递数据，最常用的方式就是通过消息传递或者将锁和Arc联合使用

// 消息通道
// 与 Go 语言内置的chan不同，Rust 是在标准库里提供了消息通道(channel)
// 你可以将其想象成一场直播，多个主播联合起来在搞一场直播，最终内容通过通道传输给屏幕前的我们
// 其中主播被称之为发送者，观众被称之为接收者，显而易见的是：一个通道应该支持多个发送者和接收者
// 多发送者 -> 单接收者，多发送者 -> 多接收者

// 多发送者，单接收者
// 标准库提供了通道std::sync::mpsc，其中mpsc是multiple producer, single consumer的缩写，代表了该通道支持多个发送者，但是只支持唯一的接收者。
fn test_mpsc() {
    // tx,rx对应发送者和接收者，它们的类型由编译器自动推导: tx.send(1)发送了整数
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    // 它们分别是mpsc::Sender<i32>和mpsc::Receiver<i32>类型
    // 由于内部是泛型实现，一旦类型被推导确定，该通道就只能传递对应类型的值, 例如此例中非i32类型的值将导致编译错误
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    // 需要使用move将tx的所有权转移到子线程的闭包中
    thread::spawn(move || {
        // send方法返回一个Result<T,E>，说明它有可能返回一个错误，例如接收者被drop导致了发送的值不会被任何人接收，此时继续发送毫无意义，因此返回一个错误最为合适
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    // 接收消息的操作rx.recv()会阻塞当前线程，直到读取到值，或者通道被关闭
    println!("receive {}", rx.recv().unwrap());
}

// 不阻塞的 try_recv 方法
// 该方法并不会阻塞线程
fn test_try_recv() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });
    // 当子线程创建成功且发送消息后，主线程会接收到Ok(1)的消息内容
    // 紧接着子线程结束，发送者也随着被drop
    // 此时接收者又会报错，但是这次错误原因有所不同：Disconnected代表发送者已经被关闭。
    // receive Err(Empty)
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
}

// 传输具有所有权的数据
// 1. 若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
// 2. 若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错
// 第二种情况
fn test_ownership() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // String底层的字符串是存储在堆上，并没有实现Copy特征
        let s = String::from("我，飞走咯!");
        // 当它被发送后，会将所有权从发送端的s转移给接收端的received
        tx.send(s).unwrap();
        // 之后s将无法被使用
        // println!("val is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 使用 for 进行循环接收
fn test_for() {
    // 主线程和子线程是并发运行的
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            // 子线程在不停的发送消息 -> 休眠 1 秒
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 当子线程运行完成时，发送者tx会随之被drop，此时for循环将被终止，最终main线程成功结束。
    for received in rx {
        // 主线程使用for循环阻塞的从rx迭代器中接收消息
        println!("Got: {}", received);
    }
}

// 使用多发送者
// 由于子线程会拿走发送者的所有权，因此我们必须对发送者进行克隆，然后让每个线程拿走它的一份拷贝:
fn test_multiple_sender() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
// 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
// 这里虽然用了clone但是并不会影响性能，因为它并不在热点代码路径中，仅仅会被执行一次
// 由于两个子线程谁先创建完成是未知的，因此哪条消息先发送也是未知的，最终主线程的输出顺序也不确定

// 消息顺序
// 上述第三点的消息顺序仅仅是因为线程创建引起的，并不代表通道中的消息是无序的，对于通道而言，消息的发送顺序和接收顺序是一致的，满足FIFO原则(先进先出)。

// 同步和异步通道
// Rust 标准库的mpsc通道其实分为两种类型：同步和异步。
// 异步通道
// 消息发送者在发送消息时都不会阻塞
fn test_async_channel() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        // 从输出还可以看出，发送之前和发送之后是连续输出的，没有受到接收端主线程的任何影响
        // 因此通过mpsc::channel创建的通道是异步通道。
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    // 主线程因为睡眠阻塞了 3 秒，因此并没有进行消息接收
    // 而子线程却在此期间轻松完成了消息的发送。等主线程睡眠结束后，才姗姗来迟的从通道中接收了子线程老早之前发送的消息。
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// 同步通道
// 与异步通道相反，同步通道发送消息是阻塞的，只有在消息被接收后才解除阻塞，例如：
fn test_sync_channel() {
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    // 主线程由于睡眠被阻塞导致无法接收消息，因此子线程的发送也一直被阻塞
    // 直到主线程结束睡眠并成功接收消息后，发送才成功
    // 发送之后的输出是在receive 1之后，说明只有接收消息彻底成功后，发送消息才算完成。
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// 消息缓存
// mpsc::sync_channel(0)
// 当你设定为N时，发送者就可以无阻塞的往通道中发送N条消息，
// 当消息缓冲队列满了后,新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的消息，那么第N+1条消息就将触发发送阻塞)。
// 事实上异步通道的缓冲上限取决于你的内存大小，不要撑爆就行。
// 因此，使用异步消息虽然能非常高效且不会造成发送线程的阻塞，但是存在消息未及时消费，最终内存过大的问题。
// 在实际项目中，可以考虑使用一个带缓冲值的同步通道来避免这种风险。

// 关闭通道
// 所有发送者被drop或者所有接收者被drop后，通道会自动关闭。

// 传输多种类型的数据
enum Fruit {
    Apple(u8),
    Orange(String),
}
// Rust 会按照枚举中占用内存最大的那个成员进行内存对齐，
// 这意味着就算你传输的是枚举中占用内存最小的成员，它占用的内存依然和最大的成员相同, 因此会造成内存上的浪费。
fn test_diff_data() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}

// 新手容易遇到的坑
fn test_message_passing() {
    use std::thread;

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        // 子线程拿走的是复制后的send的所有权，这些拷贝会在子线程结束后被drop,因此无需担心
        // 但是send本身却直到main函数的结束才会被drop。
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }
    // 发送者全部drop或接收者被drop，要结束for循环显然是要求发送者全部drop，但是由于send自身没有被drop，会导致该循环永远无法结束，最终主线程会一直阻塞。
    // 在这里drop send...
    // drop(send);

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}

// mpmc 更好的性能
// 如果你需要 mpmc(多发送者，多接收者)或者需要更高的性能，可以考虑第三方库:

// crossbeam-channel, 老牌强库，功能较全，性能较强，之前是独立的库，但是后面合并到了crossbeam主仓库中
// flume, 官方给出的性能数据某些场景要比 crossbeam 更好些
