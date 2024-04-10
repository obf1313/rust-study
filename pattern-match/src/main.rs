fn main() {
    println!("Hello, world!");

    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    // 这里使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配。
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 模式匹配要求两边的类型必须相同
    // let (x, y) = (1, 2, 3); 报错
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point)
    // 类似 let , for 和 match 都必须要求完全覆盖匹配，才能通过编译( 不可驳模式匹配 )
    // 但是对于 if let，就可以这样使用
    // if let Some(x) = some_option_value {
    //  println!("{}", x);
    // }
    // 因为 if let 允许匹配一种模式，而忽略其余的模式( 可驳模式匹配 )。
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
