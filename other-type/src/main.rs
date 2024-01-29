fn main() {
    char();
    bool();
    // 单元类型就是 ()
    // main 函数就返回这个单元类型 ()
    // println!() 的返回值也是单元类型 ()
    // 可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key
}

fn char() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    let x = '中';
    // 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}

fn bool() {
    let t = true;
    let f: bool = false;
    if f {
        println!("无意义");
    }
}
