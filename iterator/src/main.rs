use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

// For 循环与迭代器
fn test_for() {
    let arr = [1, 2, 3];
    // 数组实现了 IntoIterator 特征，Rust 通过 for 语法糖，自动把实现了该特征的数组类型转换为迭代器
    for v in arr {
        println!("{}", v);
    }
    // IntoIterator 特征拥有一个 into_iter 方法
    let arr = [1, 2, 3];
    for v in arr.into_iter() {
        println!("{}", v);
    }
    // 惰性初始化
    // 在 Rust 中，迭代器是惰性的，意味着如果你不使用它，那么它将不会发生任何事
    let v1 = vec![1, 2, 3];
    // 在 for 循环之前，我们只是简单的创建了一个迭代器 v1_iter
    // 此时不会发生任何迭代行为，只有在 for 循环开始后，迭代器才会开始迭代其中的元素，最后打印出来
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
}

// next 方法
// 迭代器之所以成为迭代器，就是因为实现了 Iterator 特征
// 要实现该特征，最主要的就是实现其中的 next 方法
// 最主要的就是实现其中的 next 方法，该方法控制如何从集合中取值，最终返回值的类型是关联类型 Item
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 省略其余有默认实现的方法
}

fn test_next() {
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    // next 方法返回的是 Option 类型，当有值时返回 Some(i32)，无值时返回 None
    // 遍历是按照迭代器中元素的排列顺序依次进行的，因此我们严格按照数组中元素的顺序取出了 Some(1)，Some(2)，Some(3)
    // 手动迭代必须将迭代器声明为 mut 可变，因为调用 next 会改变迭代器其中的状态数据（当前遍历的位置等），而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成
    // next 方法对迭代器的遍历是消耗性的
    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);

    // 模拟实现 for 循环
    let values = vec![1, 2, 3];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => {
                        println!("{}", x);
                    }
                    None => break,
                }
            },
        };
        result
    }
}

// IntoIterator 特征
// impl<I: Iterator> IntoIterator for I {
//     type Item = I::Item;
//     type IntoIter = I;

//     #[inline]
//     fn into_iter(self) -> I {
//         self
//     }
// }
fn test_into_iterator() {
    let values = vec![1, 2, 3];
    for v in values.into_iter().into_iter().into_iter() {
        println!("{}", v)
    }

    // into_iter, iter, iter_mut
    // into_iter 会夺走所有权
    // iter 是借用
    // iter_mut 是可变借用
    let values = vec![1, 2, 3];
    for v in values.into_iter() {
        println!("{}", v)
    }
    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);
    let values = vec![1, 2, 3];
    // .iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
    let _values_iter = values.iter();
    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);
    let mut values = vec![1, 2, 3];
    // .iter_mut() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&mut T)
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();
    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }
    // 输出[0, 2, 3]
    println!("{:?}", values);

    // Iterator 和 IntoIterator 的区别
    // Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 next。
    // 而 IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器。
}

// 消费者与适配器
// 消费者是迭代器上的方法，它会消费掉迭代器中的元素，然后返回其类型的值，
// 这些消费者都有一个共同的特点：在它们的定义中，都依赖 next 方法来消费元素，
// 因此这也是为什么迭代器要实现 Iterator 特征，而该特征必须要实现 next 方法的原因。

// 消费者适配器
// 只要迭代器上的某个方法 A 在其内部调用了 next 方法，那么 A 就被称为消费性适配器：因为 next 方法会消耗掉迭代器上的元素，所以方法 A 的调用也会消耗掉迭代器上的元素。
fn test_consumer_adapter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}", v1);
    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);

    // fn sum<S>(self) -> S
    // where
    //     Self: Sized,
    //     S: Sum<Self::Item>,
    // {
    //     Sum::sum(self)
    // }

    // 迭代器适配器
    // 与消费者适配器不同，迭代器适配器是惰性的，意味着你需要一个消费者适配器来收尾，最终将迭代器转换成一个具体的值：
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);

    // map 方法是一个迭代者适配器，它是惰性的，不产生任何行为，因此我们还需要一个消费者适配器进行收尾
    let v1: Vec<i32> = vec![1, 2, 3];
    // 为 v2 标注了 Vec<_> 类型，就是为了告诉 collect：请把迭代器中的元素消费掉，然后把值收集成 Vec<_> 类型
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // collect
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    // zip 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，形成 Iterator<Item=(ValueFromA, ValueFromB)> 这样的新的迭代器
    // 在此处就是形如 [(name1, age1), (name2, age2)] 的迭代器。
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);

    // 闭包作为适配器参数
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // 我们使用闭包来作为迭代器适配器的参数，它最大的好处不仅在于可以就地实现迭代器中元素的处理，还在于可以捕获环境值
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
}

// 实现 Iterator 特征
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
// 为自定义类型实现 Iterator 特征即可
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
fn test_counter() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 实现 Iterator 特征的其它方法
    // FIXME: 会报错
    // let sum: u32 = Counter::new()
    //     .zip(Counter::new().skip(1))
    //     .map(|(a, b)| a * b)
    //     .filter(|x| x % 3 == 0)
    //     .sum();
    // assert_eq!(18, sum);

    // enumerate
    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("第{}个值是{}", i, v)
    }
    // enumerate 是迭代器适配器
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v
        .iter()
        .enumerate()
        // 每两个元素剔除一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + acm);
    println!("{}", val);
}
