use std::io;

fn main() {
    // 数组 array 是存储在栈上
    // 动态数组 Vector 是存储在堆上
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 某个值重复出现 N 次的数组
    let b = [3; 5];
    // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];

    // 越界访问
    println!("Please enter an array index.");
    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    // 基本类型在Rust中赋值是以Copy的形式
    // let array = [String::from("rust is good!"); 8];
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    // # 好像是换行
    println!("{:#?}", array);

    // 数组切片
    // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        println!("{:?}", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
// 在 Rust 中，最常用的数组有两种，第一种是速度很快但是长度固定的 array，第二种是可动态增长的但是有性能损耗的 Vector
