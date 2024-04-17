use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // 这里的 Result 可能包含一个 Config 实例，也可能包含一条错误信息 &static str
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // clone 直接完整的复制目标数据，无需被所有权、借用等问题所困扰
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

// 将主体逻辑( 例如业务逻辑 )从 main 中分离出去，这样 main 函数就保留主流程调用
// 我们的程序无需返回任何值，但是为了满足 Result<T,E> 的要求，因此使用了 Ok(()) 返回一个单元类型 ()
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
