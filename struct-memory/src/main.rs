#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    // 是因为要打印对象吗？
    // 使用 #[derive(Debug)] 对结构体进行了标记
    // 才能使用 println!("{:?}", s)
    // 不想实现 Display 特征，才用的 {:?}
    println!("{:?}", f1);
    // 当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 {:#?} 来替代 {:?}
    println!("{} is {} bytes long", f1_name, f1_length);
}
