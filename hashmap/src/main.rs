// HashMap 并没有包含在 Rust 的 prelude 中
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    test_hash_update();
}

fn test_hash_create() {
    // 创建 HashMap
    let mut my_gems = HashMap::new();
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);
    // 如果预先知道要存储的 KV 对个数，可以使用 HashMap::with_capacity(capacity)
    // 创建指定大小的 HashMap，避免频繁的内存分配和拷贝，提升性能

    // 使用迭代器和 collect 方法创建
    // 笨办法
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }
    println!("{:?}", teams_map);
    // 先将 Vec 转为迭代器，接着通过 collect 方法，将迭代器中的元素收集后，转成 HashMap
    // 需要通过类型标注 HashMap<_,_> 来告诉编译器：请帮我们收集为 HashMap 集合类型
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    // 所有权转移
    // 若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
    // 若没实现 Copy 特征，所有权将被转移给 HashMap 中
    let name = String::from("Sunface");
    let age = 18;

    // let mut handsome_boys = HashMap::new();
    // handsome_boys.insert(name, age);
    // 报错
    // 在 insert 时，它的所有权被转移给 handsome_boys，所以最后在使用时，会遇到这个无情但是意料之中的报错
    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age);
    // 如果你使用引用类型放入 HashMap 中，请确保该引用的生命周期至少跟 HashMap 活得一样久
    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);
    // 就通过 drop 函数手动将 name 字符串从内存中移除
    std::mem::drop(name);
    // println!("因为过于无耻，{:?}已经被除名", handsome_boys);
    // println!("还有，他的真实年龄远远不止{}岁", age);
}

fn test_hash_select() {
    // 查询 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // 通过 get 方法可以获取元素
    let score: Option<&i32> = scores.get(&team_name);
    // get 方法返回一个 Option<&i32> 类型：当查询不到时，会返回一个 None，查询到时返回 Some(&i32)
    // &i32 是对 HashMap 中值的借用，如果不使用借用，可能会发生所有权的转移
    // 如果我们想直接获得值类型的 score 该怎么办
    // TODO: 查询下 Option 的 copied 方法和 unwrap_or 方法的含义及该如何使用。
    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);

    // 通过循环的方式依次遍历 KV 对
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn test_hash_update() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    // 不存在，插入5
    assert_eq!(*v, 5);
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    // 已经存在，因此50没有插入
    assert_eq!(*v, 5);

    // 在已有值的基础上更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        // or_insert 返回了 &mut v 引用，因此可以通过该可变引用直接修改 map 中对应的值
        let count = map.entry(word).or_insert(0);
        // 解引用 *count
        *count += 1;
    }
    println!("{:?}", map);
}

// 哈希函数
// 一个类型能否作为 Key 的关键就是是否能进行相等比较，或者说该类型是否实现了 std::cmp::Eq 特征
// f32 和 f64 浮点数，没有实现 std::cmp::Eq 特征，因此不可以用作 HashMap 的 Key。
// 有哈希函数：通过它把 Key 计算后映射为哈希值，然后使用该哈希值来进行存储、查询、比较等操作。

// 高性能三方库
// 若性能测试显示当前标准库默认的哈希函数不能满足你的性能需求，就需要去 crates.io 上寻找其它的哈希函数实现，使用方法很简单
use std::hash::BuildHasherDefault;
// 引入第三方的哈希函数
// use twox_hash::XxHash64;

fn test_hash_func() {
    // 指定HashMap使用第三方的哈希函数XxHash64
    // let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    // hash.insert(42, "the answer");
    // assert_eq!(hash.get(&42), Some(&"the answer"));
}
// 目前，HashMap 使用的哈希函数是 SipHash，它的性能不是很高，但是安全性很高。SipHash 在中等大小的 Key 上，性能相当不错，但是对于小型的 Key （例如整数）或者大型 Key （例如字符串）来说，性能还是不够好。
// 若你需要极致性能，例如实现算法，可以考虑这个库：ahash。
