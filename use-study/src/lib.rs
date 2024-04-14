// 基本引入方式
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// 相对路径引入模块中的函数
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
// }

// 引入模块还是函数
// 优先使用最细粒度(引入函数、结构体等)的引用方式，如果引起了某种麻烦(例如前面两种情况)，再使用引入模块的方式。

// 避免同名引用
// use std::fmt;
// use std::io;

// 使用父模块的方式来调用
// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }
// as 别名引用
use std::io::Result as IoResult;

// 引入项再导出
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// 使用 pub use 即可实现
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 使用第三方包 crates.io，lib.rs
// 1. 修改 Cargo.toml 文件，在 [dependencies] 区域添加一行：rand = "0.8.3"
// 2. 此时，如果你用的是 VSCode 和 rust-analyzer 插件，该插件会自动拉取该库，你可能需要等它完成后，再进行下一步（VSCode 左下角有提示）
// use rand::Rng;
// fn test() {
//     let secret_number = rand::thread_rng().gen_range(1..101);
// }

// 使用 {} 简化引入方式
// use std::collections::{BTreeMap, HashMap, HashSet};
// use std::{cmp::Ordering, io};
// 同时引入模块和模块中的项
use std::io::{self, Write};
// use self::xxx，表示加载当前模块中的 xxx。此时 self 可省略
// use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy

// 使用 * 引入模块下的所有项
// 当使用 * 来引入的时候要格外小心，因为你很难知道到底哪些被引入到了当前作用域中，有哪些会和你自己程序中的名称相冲突
use std::collections::*;

// 受限的可见性
// 在 Rust 中，包是一个模块树，我们可以通过 pub(crate) item
// item 虽然是对外可见的，但是只在当前包内可见，外部包无法引用到该 item

// 所以，如果我们想要让某一项可以在整个包中都可以被使用，那么有两种办法：
// 1. 在包根中定义一个非 pub 类型的 X(父模块的项对子模块都是可见的，因此包根中的项对模块树上的所有模块都可见)
// 2. 在子模块中定义一个 pub 类型的 Y，同时通过 use 将其引入到包根
// mod a {
//     pub mod b {
//         pub fn c() {
//             println!("{:?}", crate::X);
//         }

//         #[derive(Debug)]
//         pub struct Y;
//     }
// }

// #[derive(Debug)]
// struct X;
// use a::b::Y;
// fn d() {
//     println!("{:?}", Y);
// }

// 希望对于某些特定的模块可见，但是对于其他模块又不可见
// 目标：`a` 导出 `I`、`bar` and `foo`，其他的不导出
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        // 通过 pub(in crate::a) 的方式，我们指定了模块 c 和常量 J 的可见范围都只是 a 模块中，a 之外的模块是完全访问不到它们的。
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}

// 限制可见性语法
// pub 意味着可见性无任何限制
// pub(crate) 表示在当前包可见
// pub(self) 在当前模块可见
// pub(super) 在父模块可见
// pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块

// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前包中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // 报错！ `private_nested` 是私有的
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}
