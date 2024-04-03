// Rust 在编译的时候会扫描代码，变量声明后未使用会以 warning 警告的形式进行提示
#![allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // 告诉编译器该函数尚未实现
    unimplemented!()
}

fn main() {
    println!("Hello, world!");
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(&mut f1);

    let my_name = "Pascal";
    // greet(my_name);
    // slice();
    // str_opt();
    utf8();
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

// 切片
fn slice() {
    let s = String::from("hello world");
    // 切片：[开始索引..终止索引]
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);

    let s1 = String::from("hello");
    // 如果你想从索引 0 开始，可以使用如下的方式，这两个是等效的
    let slice = &s[0..2];
    let slice = &s[..2];
    // 如果你的切片想要包含 String 的最后一个字节，则可以这样使用
    let len = s.len();
    let slice = &s[4..len];
    let slice = &s[4..];
    // 截取完整的 String 切片
    let slice = &s[0..len];
    let slice = &s[..];
    // 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃
    let s = "中国人";
    let a = &s[0..2];
    println!("{}", a);
    // 字符串切片的类型标识是 &str
    let mut s = String::from("hello world");
    let word = first_world(&s);
    // 当我们已经有了可变借用时，就无法再拥有不可变的借用。
    // 因为 clear 需要清空改变 String，因此它需要一个可变借用（利用 VSCode 可以看到该方法的声明是 pub fn clear(&mut self) ，参数是对自身的可变借用 ）
    // s.clear();
    // 而之后的 println! 又使用了不可变借用，也就是在 s.clear() 处可变借用与不可变借用试图同时生效，因此编译无法通过
    println!("the first word is: {}", word);

    // 其他集合也有切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // 字符串字面量是切片
    let s = "Hello, world!";
    // 该切片指向了程序可执行文件中的某个点，这也是为什么字符串字面量是不可变的
    // 因为 &str 是一个不可变引用
    let s: &str = "Hello, world!";
}

fn first_world(s: &String) -> &str {
    &s[..1]
}

fn str() {
    // Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)
    // 将 String 类型转为 &str 类型
    let s = String::from("hello,world!");
    // 因为 deref 隐式强制转换
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    // 使用索引的方式访问字符串的某个字符或者子串是很正常的行为，但是在 Rust 中就会报错
    let s1 = String::from("hello");
    // let h = s1[0];
    // 因为索引操作，我们总是期望它的性能表现是 O(1)
    // 然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符。
}
fn say_hello(s: &str) {
    println!("{}", s);
}

/** 操作字符串 */
fn str_opt() {
    // 追加
    let mut s = String::from("hello");
    // 好好笑，要用双引号
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    // 好好笑，还要用单引号
    s.push('!');
    println!("最佳字符 push() -> {}", s);

    // 替换
    // replace: 该方法可适用于 String 和 &str 类型
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
    // replacen: 该方法可适用于 String 和 &str 类型
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    // replace_range: 该方法仅适用于 String 类型。
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // 删除
    // pop
    // 该方法是直接操作原来的字符串。
    let mut string_pop = String::from("rust pop 中文！");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    // remove
    // 该方法是直接操作原来的字符串。
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 如果参数所给的位置不是合法的字符边界
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
    // truncate
    // 该方法是直接操作原来的字符串
    // 删除字符串中从指定位置开始到结尾的全部字符
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    // clear
    // 清空字符串
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 连接
    // 使用 + 或者 += 连接字符串
    // + 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰
    let string_append = String::from("hello");
    let string_rust = String::from("rust");
    // &string_rust 会自动解引为 &str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("连接字符串 + -> {}", result);
    // 使用 format! 连接字符串
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    // 字符串转义
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    // \u 可以输出一个 unicode 字符
    let unicode_code_point = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_code_point, character_name
    );
    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
    can span multiple lines.
    The linebreak and indentation here -> \
    <- can be escaped too!";
    println!("{}", long_string);
    // 不转义
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \x{211D}";
    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said:"There is no escape!""#;
    println!("{}", quotes);
    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

fn add() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    // 将 String 类型的 s1 与 &str 类型的 s2 进行相加，最终得到 String 类型的 s3
    // s1 所有权 转移到 s3
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    // 报错
    // println!("{}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
}

fn utf8() {
    for c in "中国人".chars() {
        println!("{}", c);
    }

    for b in "中国人".bytes() {
        println!("{}", b);
    }

    // 想要准确的从 UTF-8 字符串中获取子串是较为复杂的事情，例如想要从 holla中国人नमस्ते 这种变长的字符串中取出某一个子串，使用标准库你是做不到的。 你需要在 crates.io 上搜索 utf8 来寻找想要的功能。
    // 可以考虑尝试下这个库：utf8_slice。
}

fn rust_drop() {
    // Rust 也提供了一个释放内存的函数： drop，但是不同的是，其它语言要手动调用 free 来释放每一个变量占用的内存
    // 而 Rust 则在变量离开作用域时，自动调用 drop 函数: 上面代码中，Rust 在结尾的 } 处自动调用 drop
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    // len() 返回字符串的长度
    let length = s.len();
    (s, length)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn structFn() {
    // Rust 不支持将某个结构体某个字段标记为可变
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("a@example.com");

    // 因此在上面代码中，user1 的部分字段所有权被转移到 user2 中
    // username 字段发生了所有权转移，作为结果，user1 无法再被使用
    // username 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用
    let user2 = User {
        email: String::from("www@email.com"),
        // 结构体更新语法
        // ..user1 必须在结构体的尾部使用。
        ..user1
    };

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.active);
    // 下面这行会报错
    // :? 到底是啥语法
    // println!("{:?}", user1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 元组结构体
fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 单元结构体
fn unit_like_struct() {
    // 如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体：
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // impl SomeTrait for AlwaysEqual {}
}
