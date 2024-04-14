fn main() {
    test_diff_type();
    test_vec_fn();
}

fn test_vec() {
    // 创建动态数组
    // v 被显式地声明了类型 Vec<i32>
    // 这是因为 Rust 编译器无法从 Vec::new() 中得到任何关于类型的暗示信息
    let v: Vec<i32> = Vec::new();
    // 编译器通过 v.push(1)，推测出 v 中的元素类型是 i32
    let mut v = Vec::new();
    v.push(1);
    // 如果预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity) 创建动态数组
    // 使用宏 vec! 来创建数组
    let v = vec![1, 2, 3];

    // 更新 Vector
    // 向数组尾部添加元素
    let mut v = Vec::new();
    v.push(1);
    // Vector 与其元素共存亡
    {
        let v = vec![1, 2, 3];
        // ...
    }
    // <- v超出作用域并在此处被删除
    // 但是当 Vector 中的元素被引用后，事情可能会没那么简单

    // 从 Vector 中读取元素
    // 通过下标索引访问。
    // 使用 get 方法。
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    // 下标索引与 .get 的区别
    let v = vec![1, 2, 3, 4, 5];

    // &v[100] 的访问方式会导致程序无情报错退出
    let does_not_exist = &v[100];
    // v.get 就不会，它在内部做了处理，有值的时候返回 Some(T)，无值的时候返回 None
    let does_not_exist = v.get(100);

    // 同时借用多个数组元素
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来
    // 这种情况下，之前的引用显然会指向一块无效的内存
    v.push(6);
    // 如果 first 在 v.push 之后不再使用，那么该段代码可以成功编译
    // println!("The first element is: {first}");

    // 迭代遍历 Vector 中的元素
    let v = vec![100, 32, 57];
    // 每次下标访问都会触发数组边界检查
    for i in &v {
        println!("{i}");
    }
    // 也可以在迭代过程中，修改 Vector 中的元素
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        // *i 解引用
        *i += 10
    }
}

// 存储不同类型的元素
// 通过使用枚举类型和特征对象来实现不同类型元素的存储
// 通过枚举
// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn test_diff_type() {
//     let v = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];

//     for ip in v {
//         show_addr(ip)
//     }
// }

// fn show_addr(ip: IpAddr) {
//     println!("{:?}", ip);
// }

// 通过特征对象
trait IpAddr {
    fn display(&self);
}

struct V4(String);
// 为 V4 和 V6 都实现了特征 IpAddr
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
// 为 V4 和 V6 都实现了特征 IpAddr
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}
fn test_diff_type() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        // 然后将它俩的实例用 Box::new 包裹后，存在了数组 v 中
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
// 特征对象数组要比枚举数组常见很多

// Vector 常用方法
// 如果容量不足就会导致 vector 扩容
// 目前的策略是重新申请一块 2 倍大小的内存，再将所有元素拷贝到新的内存位置，同时更新指针数据
fn test_vec_fn() {
    let mut v = Vec::with_capacity(10);
    // 附加数据到 v
    v.extend([1, 2, 3]);
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());
    // 调整 v 的容量，至少要有 100 的容量
    v.reserve(100);
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );
    // 释放剩余的容量，一般情况下，不会主动去释放容量
    // 将无用的容量释放掉
    v.shrink_to_fit();
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    // Vector 常见的一些方法
    let mut v = vec![1, 2];
    // 检查 v 是否为空
    assert!(!v.is_empty());
    // 在指定索引插入数据，索引值不能大于 v 的长度，v: [1, 2, 3]
    v.insert(2, 3);
    // 移除指定位置的元素并返回, v: [1, 3]
    assert_eq!(v.remove(1), 2);
    // 删除并返回 v 尾部的元素，v: [1]
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(1));
    // 记得 pop 方法返回的是 Option 枚举值
    assert_eq!(v.pop(), None);
    // 清空 v, v: []
    v.clear();
    let mut v1 = [11, 22].to_vec();
    // append 操作会导致 v1 清空数据，增加可变声明
    // 将 v1 中的所有元素附加到 v 中, v1: []
    v.append(&mut v1);
    // 截断到指定长度，多余的元素被删除, v: [11]
    v.truncate(1);
    // 保留满足条件的元素，即删除不满足条件的元素
    // *x 解引用
    v.retain(|x| *x > 10);
    let mut v = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
    let mut m: Vec<_> = v.drain(1..=3).collect();
    // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
    let v2 = m.split_off(1);
    // 也可以像数组切片的方式获取 vec 的部分元素
    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);
}

// Vector 的排序
fn test_vec_sort() {
    // 在 rust 里，实现了两种排序算法，分别为稳定的排序 sort 和 sort_by
    // 以及非稳定排序 sort_unstable 和 sort_unstable_by。
    // 非稳定 并不是指排序算法本身不稳定，而是指在排序过程中对相等元素的处理方式
    // 在 稳定 排序算法里，对相等的元素，不会对其进行重新排序
    // 而在 不稳定 的算法里则不保证这点
    // 非稳定 排序的算法的速度会优于 稳定 排序算法，同时，稳定 排序还会额外分配原数组一半的空间

    // 整数数组的排序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // 浮点数数组的排序
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // 报错
    // 在浮点数当中，存在一个 NAN 的值，这个值无法与其他的浮点数进行对比
    // vec.sort_unstable();
    // assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
    // 如果我们确定在我们的浮点数数组当中，不包含 NAN 值，那么我们可以使用 partial_cmp 来作为大小判断的依据
    // TODO: unwrap 是啥
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);

    // 对结构体数组进行排序
    // #[derive(Debug)]
    // struct Person {
    //     name: String,
    //     age: u32,
    // }

    // impl Person {
    //     fn new(name: String, age: u32) -> Person {
    //         Person { name, age }
    //     }
    // }
    // let mut people = vec![
    //     Person::new("Zoe".to_string(), 25),
    //     Person::new("Al".to_string(), 60),
    //     Person::new("John".to_string(), 1),
    // ];
    // 定义一个按照年龄倒序排序的对比函数
    // people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
    // println!("{:?}", people);

    // 排序需要我们实现 Ord 特性，那么如果我们把我们的结构体实现了该特性，是否就不需要我们自定义对比函数了呢
    // 是，但不完全是，实现 Ord 需要我们实现 Ord、Eq、PartialEq、PartialOrd 这些属性
    // 好消息是，你可以 derive 这些属性
    #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();

    println!("{:?}", people);

    // 需要 derive Ord 相关特性，需要确保你的结构体中所有的属性均实现了 Ord 相关特性，否则会发生编译错误
    // derive 的默认实现会依据属性的顺序依次进行比较
    // 如上述例子中，当 Person 的 name 值相同，则会使用 age 进行比较
}
