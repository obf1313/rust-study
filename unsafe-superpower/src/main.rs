use std::{
    slice::{self, from_raw_parts},
    str::from_utf8_unchecked,
};

fn main() {
    println!("Hello, world!");
    create_raw_pointer();
    unsafe {
        dangerous();
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// 解引用裸指针
// 裸指针(raw pointer，又称原生指针) 在功能上跟引用类似，同时它也需要显式地注明可变性。
// 但是又和引用有所不同，裸指针长这样: *const T 和 *mut T，它们分别代表了不可变和可变。

// 三种类似指针的概念
// 引用、智能指针和裸指针
// 与前两者不同，裸指针：
// 1. 可以绕过 Rust 的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
// 2. 并不能保证指向合法的内存
// 3. 可以是 null
// 4. 没有实现任何自动的回收 (drop)

fn create_raw_pointer() {
    // 基于引用创建裸指针
    let mut num = 5;
    // as 可以用于强制类型转换
    // 我们将引用 &num / &mut num 强转为相应的裸指针 *const i32 / *mut i32
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为
    unsafe {
        println!("r1 is: {}", *r1);
    }

    // 基于内存地址创建裸指针
    let address = 0x012345usize;
    let r = address as *const i32;

    // 如果真的要使用内存地址，也是类似下面的用法，先取地址，再使用，而不是凭空捏造一个地址：
    // 获取字符串的内存地址和长度
    fn get_memory_location() -> (usize, usize) {
        let string = "Hello World!";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
    }

    // 在指定的内存地址读取字符串
    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // 有啥，没懂？
    let message = get_str_at_location(1000, 10);

    // 使用 * 解引用
    let a = 1;
    let b: *const i32 = &a as *const i32;
    // 使用了隐式的转换方式 let c: *const i32 = &a
    // 建议使用 as 来转换，因为这种显式的方式更有助于提醒用户
    let c: *const i32 = &a;
    unsafe {
        // 我们需要使用 unsafe 来包裹解引用的逻辑
        println!("{}", *c);
    }

    // 基于智能指针创建裸指针
    let a: Box<i32> = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    let c: *const i32 = Box::into_raw(a);
}

// 调用 unsafe 函数或方法
// unsafe 函数从外表上来看跟普通函数并无区别，唯一的区别就是它需要使用 unsafe fn 来进行定义
// 使用 unsafe 声明的函数时，一定要看看相关的文档，确定自己没有遗漏什么。
// 在 unsafe 函数体中使用 unsafe 语句块是多余的行为。
unsafe fn dangerous() {}

// 用安全抽象包裹 unsafe 代码
// 一个函数包含了 unsafe 代码不代表我们需要将整个函数都定义为 unsafe fn。
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid], &mut slice[mid..])
// }
// 虽然 split_at_mut 使用了 unsafe，但我们无需将其声明为 unsafe fn，这种情况下就是使用安全的抽象包裹 unsafe 代码，这里的 unsafe 使用是非常安全的，因为我们从合法数据中创建了的合法指针
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // as_mut_ptr 会返回指向 slice 首地址的裸指针 *mut i32
    let ptr = slice.as_mut_ptr();
    // 那这段代码我们怎么保证 unsafe 中使用的裸指针 ptr 和 ptr.add(mid) 是合法的呢？秘诀就在于 assert!(mid <= len); ，通过这个断言，我们保证了裸指针一定指向了 slice 切片中的某个元素，而不是一个莫名其妙的内存地址。
    assert!(mid <= len);
    // 由于 slice::from_raw_parts_mut 使用裸指针作为参数，因此它是一个 unsafe fn，我们在使用它时，就必须用 unsafe 语句块进行包裹，类似的，.add 方法也是如此(还是那句话，不要将无关的代码包含在 unsafe 语句块中)。
    unsafe {
        (
            // slice::from_raw_parts_mut 函数通过指针和长度来创建一个新的切片，简单来说，该切片的初始地址是 ptr，长度为 mid
            slice::from_raw_parts_mut(ptr, mid),
            // ptr.add(mid) 可以获取第二个切片的初始地址，由于切片中的元素是 i32 类型，每个元素都占用了 4 个字节的内存大小，
            // 因此我们不能简单的用 ptr + mid 来作为初始地址，而应该使用 ptr + 4 * mid，但是这种使用方式并不安全，因此 .add 方法是最佳选择
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// FFI
// FFI（Foreign Function Interface）可以用来与其它语言进行交互
// 如何调用 C 标准库中的 abs 函数
// 其中 "C" 定义了外部函数所使用的应用二进制接口ABI (Application Binary Interface)：ABI 定义了如何在汇编层面来调用该函数。
// 在所有 ABI 中，C 语言的是最常见的。
extern "C" {
    fn abs(input: i32) -> i32;
}

fn test_c() {
    // extern 必须使用 unsafe 才能进行进行调用，原因在于其它语言的代码并不会强制执行 Rust 的规则
    // 因此 Rust 无法对这些代码进行检查，最终还是要靠开发者自己来保证代码的正确性和程序的安全性。
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 在其它语言中调用 Rust 函数
// 我们可以使用 extern 来创建一个接口，其它语言可以通过该接口来调用相关的 Rust 函数。
// 但是此处的语法与之前有所不同，之前用的是语句块，而这里是在函数定义时加上 extern 关键字，
// 当然，别忘了指定相应的 ABI：
#[no_mangle]
// #[no_mangle]，用于告诉 Rust 编译器：不要乱改函数的名称。
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// 实现 unsafe 特征
// unsafe 特征的声明
unsafe trait Foo {
    // 方法列表
}

// Send 特征标记为 unsafe 是因为 Rust 无法验证我们的类型是否能在线程间安全的传递，因此就需要通过 unsafe 来告诉编译器，它无需操心，剩下的交给我们自己来处理。
unsafe impl Foo for i32 {
    // 实现相应的方法
}

// 访问 union 中的字段
// 主要用于跟 C 代码进行交互。
// 访问 union 的字段是不安全的，因为 Rust 无法保证当前存储在 union 实例中的数据类型。
#[repr(C)]
// union 的使用方式跟结构体确实很相似，但是前者的所有字段都共享同一个存储空间
// 意味着往 union 的某个字段写入值，会导致其它字段的值会被覆盖。
// 关于 union 的更多信息，可以在这里查看。
// https://doc.rust-lang.org/reference/items/unions.html
union MyUnion {
    f1: u32,
    f2: f32,
}

// 一些实用工具(库)

// rust-bindgen 和 cbindgen
// 这两个库可以帮我们自动生成相应的接口
// 其中 rust-bindgen 用于在 Rust 中访问 C 代码，而 cbindgen则反之。
// 下面以 rust-bindgen 为例，来看看如何自动生成调用 C 的代码，首先下面是 C 代码:
// typedef struct Doggo {
//     int many;
//     char wow;
// } Doggo;

// void eleven_out_of_ten_majestic_af(Doggo* pupper);
// 下面是自动生成的可以调用上面代码的 Rust 代码：
/* automatically generated by rust-bindgen 0.99.9 */
#[repr(C)]
pub struct Doggo {
    pub many: ::std::os::raw::c_int,
    pub wow: ::std::os::raw::c_char,
}
extern "C" {
    pub fn eleven_out_of_ten_majestic_af(pupper: *mut Doggo);
}

// cxx
// 如果需要跟 C++ 代码交互，非常推荐使用 cxx，它提供了双向的调用，最大的优点就是安全：是的，你无需通过 unsafe 来使用它！

// Miri
// miri 可以生成 Rust 的中间层表示 MIR，对于编译器来说，我们的 Rust 代码首先会被编译为 MIR ，然后再提交给 LLVM 进行处理。

// TODO: MIR 啥玩意啊
// 可以通过 rustup component add miri 来安装它，并通过 cargo miri 来使用，同时还可以使用 cargo miri test 来运行测试代码。

// miri 可以帮助我们检查常见的未定义行为(UB = Undefined Behavior)，以下列出了一部分:

// 1. 内存越界检查和内存释放后再使用(use-after-free)
// 2. 使用未初始化的数据
// 3. 数据竞争
// 4. 内存对齐问题

// 但是需要注意的是，它只能帮助识别被执行代码路径的风险，那些未被执行到的代码是没办法被识别的。

// Clippy
// 官方的 clippy 检查器提供了有限的 unsafe 支持，虽然不多，但是至少有一定帮助。例如 missing_safety_docs 检查可以帮助我们检查哪些 unsafe 函数遗漏了文档。
// 需要注意的是： Rust 编译器并不会默认开启所有检查，大家可以调用 rustc -W help 来看看最新的信息。

// Prusti
// prusti 需要大家自己来构建一个证明，然后通过它证明代码中的不变量是正确被使用的，当你在安全代码中使用不安全的不变量时，就会非常有用。具体的使用文档见这里。

// 模糊测试(fuzz testing)
// 在 Rust Fuzz Book 中列出了一些 Rust 可以使用的模糊测试方法。
// 同时，我们还可以使用 rutenspitz 这个过程宏来测试有状态的代码，例如数据结构。
