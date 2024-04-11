fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = PointOther { x: 1, y: 1.1 };
    let p = Point { x: 5, y: 10 };
    test_largest();
    // println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut num = &list[0];

    for item in list.iter() {
        if item > num {
            num = item;
        }
    }

    num
}

fn test_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// 结构体中使用泛型
// struct Point<T> {
//     x: T,
//     y: T,
// }

struct PointOther<T, U> {
    x: T,
    y: U,
}

// 枚举中使用泛型
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 方法中使用泛型
// impl<T>，只有提前声明了，我们才能在Point<T>中使用它
// 这里的 Point<T> 不再是泛型声明，而是一个完整的结构体类型
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

struct Point<T, U> {
    x: T,
    y: U,
}

// T,U 是定义在结构体 Point 上的泛型参数，V,W 是单独定义在方法 mixup 上的泛型参数，它们并不冲突
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// const 泛型
// N 这个泛型参数，它是一个基于值的泛型参数
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn test_const() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

// 泛型的性能
// 当 Rust 编译这些代码的时候，它会进行单态化。编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：一种对应 i32 另一种对应 f64。
// 编译器生成的单态化版本的代码看起来像这样：
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn test_generic() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
