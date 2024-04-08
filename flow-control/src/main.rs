fn main() {
    let condition = true;
    // if 语句块是表达式
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // for 循环
    // 使用 for 时我们往往使用集合的引用形式
    // for 元素 in 集合
    for i in 1..=5 {
        print!("{}", i);
    }
    // 对于实现了 copy 特征的数组(例如 [i32; 10] )而言， for item in arr 并不会把 arr 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 arr 。

    // 如果想在循环中，修改该元素，可以使用 mut 关键字
    // for item in &mut collection {}

    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    // _ 的含义是忽略该值或者类型的意思
    for _ in 0..10 {
        // ..
    }

    // 两种循环方式优劣对比
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
    }

    for item in collection {}

    // 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
    // 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险( 由于所有权限制，在访问过程中，数据并不会发生变化)。

    // continue，跳过当前当此循环，开始下次循环
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    // break，可以直接跳出整个循环
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }

    // while
    let mut n = 0;
    while n <= 5 {
        println!("{}!", n);
        n = n + 1;
    }
    println!("done!");

    // loop
    // break 可以单独使用，也可以带一个返回值，有些类似 return
    // loop 是一个表达式，因此可以返回一个值
    loop {
        if n > 5 {
            break;
        }
        println!("{}!", n);
        n += 1;
    }
    println!("done!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
}
