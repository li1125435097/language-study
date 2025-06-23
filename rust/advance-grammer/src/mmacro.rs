#[macro_export]
// 宏的定义
macro_rules! greet {
    // 模式匹配
    ($name:expr) => {
        // 宏的展开
        println!("Hello, {}!", $name);
    };
}
