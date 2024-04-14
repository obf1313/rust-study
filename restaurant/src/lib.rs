// 模块与文件分离
// 告诉 Rust 从另一个和模块 front_of_house 同名的文件中加载该模块的内容
mod front_of_house;
// 使用绝对路径的方式来引用 hosting 模块
pub use crate::front_of_house::hosting;

// 餐厅前厅，用于吃饭
// 使用 mod 关键字来创建新模块，后面紧跟着模块名称
// mod front_of_house {
// 模块可以嵌套，这里嵌套的原因是招待客人和服务都发生在前厅，因此我们的代码模拟了真实场景
// 模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等
// 所有模块均定义在同一个文件中
// pub 关键字
// pub mod hosting {
//     // pub 关键字
//     pub fn add_to_waitlist() {}
//     fn seat_at_table() {}
// }

//     pub mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

pub fn eat_at_restaurant() {
    // 绝对路径
    // 从包根开始，路径名以包名或者 crate 作为开头
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    // 从当前模块开始，以 self，super 或当前模块的标识符作为开头
    front_of_house::hosting::add_to_waitlist();
}
// 不过，如果不确定哪个（绝对、相对）好，你可以考虑优先使用绝对路径

// 代码可见性
// 同一个模块内的代码自然不存在私有化问题
// 父模块完全无法访问子模块中的私有项，但是子模块却可以访问父模块、父父..模块的私有项

// 使用 super 引用模块
// fn serve_order() {}

// 厨房模块
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// 使用 self 引用模块
// self 其实就是引用自身模块中的项
fn serve_order() {
    //
    self::back_of_house::cook_order()
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::serve_order();
    }

    pub fn cook_order() {}
}

// 结构体和枚举的可见性
// 将结构体设置为 pub，但它的所有字段依然是私有的
// 将枚举设置为 pub，它的所有字段也将对外可见

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }
