fn main() {
    // print!，println!，format!
    // print! 将格式化文本输出到标准输出，不带换行符
    // println! 同上，但是在行的末尾添加换行符
    // format! 将格式化文本输出到 String 字符串
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");

    // eprint!，eprintln!
    // 仅应该被用于输出错误信息和进度信息，其它场景都应该使用 print! 系列
    eprintln!("Error: Could not complete task");

    // {} 与 {:?}
    // {} 适用于实现了 std::fmt::Display 特征的类型，用来以更优雅、更友好的方式格式化文本，例如展示给用户
    // {:?} 适用于实现了 std::fmt::Debug 特征的类型，用于调试场景
    // {:#?} 与 {:?} 几乎一样，唯一的区别在于它能更优美地输出内容

    // 位置参数
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
                              // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{1}{}{0}{}", 1, 2); // => 2112

    // 具名参数
    // 带名称的参数必须放在不带名称参数的后面
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"

    // 格式化参数
    let v = 3.1415926;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);

    // 宽度
    // 宽度用来指示输出目标的长度，如果长度不够，则进行填充和对齐
    // 字符串填充
    //-----------------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------
    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);

    // 数字填充:符号和 0
    // 数字格式化默认也是使用空格进行填充，但与字符串左对齐不同的是，数字是右对齐
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);

    // 对齐
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");
    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");

    // 精度
    // 精度可以用于控制浮点数的精度或者字符串的长度
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);
    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");

    // 进制
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);
    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);
    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);

    // 指数
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9

    // 指针地址
    let v = vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050

    // 转义
    // 有时需要输出 {和}，但这两个字符是特殊字符，需要进行转义：
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    // => Hello "{World}"
    println!(" Hello \"{{World}}\" ");
    // 下面代码会报错，因为占位符{}只有一个右括号}，左括号被转义成字符串的内容
    // println!(" {{ Hello } ");
    // 也不可使用 '\' 来转义 "{}"
    // println!(" \{ Hello \} ")

    // 在格式化字符串时捕获环境中的值（Rust 1.58 新增）
    // 它只能捕获普通的变量，对于更复杂的类型（例如表达式），可以先将它赋值给一个变量或使用以前的 name = expression 形式的格式化参数
    // 目前除了 panic! 外，其它接收格式化参数的宏，都可以使用新的特性。
    // 对于 panic! 而言，如果还在使用 2015版本 或 2018版本，那 panic!("{ident}") 依然会被当成 正常的字符串来处理，同时编译器会给予 warn 提示。
    // 而对于 2021版本 ，则可以正常使用:
    let person = get_person();
    panic!("Hello, {person}!");
}

fn get_person() -> String {
    String::from("sunface")
}
