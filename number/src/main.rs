fn main() {
    println!("Hello, world!");
    // int_over();
    // float_num();
    // float_other();
    // nan();
    // num_operation();
    // bit_operation();
    range();
}

fn int_over() {
    // 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    // 如果使用 checked_* 方法时发生溢出，则返回 None 值
    // 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    // 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，例如:
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b); // 19
}

fn float_num() {
    // 浮点数往往是你想要数字的近似表达
    // 浮点数在某些特性上是反直觉的
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 和 js 一样啥
    // assert!(0.1 + 0.2 == 0.3);
    // 可以考虑用这种方式
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);
}

fn float_other() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // 断言失败
    assert!(xyz.0 + xyz.1 == xyz.2);
}

fn nan() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
    // 代码会崩溃
    // assert_eq!(x, x);
}

fn num_operation() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 10;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;
    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );
    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0])
}

fn bit_operation() {
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;
    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {} ", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);
    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn range() {
    // 例如 1..5，生成从 1 到 4 的连续数字，不包含 5 ；1..=5，生成从 1 到 5 的连续数字，包含 5
    for i in 1..=5 {
        println!("{}", i)
    }
    for i in 'a'..='z' {
        println!("{}", i);
    }
}
