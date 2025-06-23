#[macro_use] extern crate nickel;

use nickel::Nickel;

// 定义主函数
fn main() {
    // 创建一个新的Nickel服务器
    let mut server = Nickel::new();

    // 使用路由器，当访问任意路径时，调用sayHello函数
    server.utilize(router! {
        get "**" => |_req, _res| {
            sayHello()
        }
    });

    // 监听127.0.0.1的6767端口
    server.listen("127.0.0.1:6767");
}

// 定义一个函数，返回一个字符串
fn sayHello() -> &'static str {
    // 返回一个字符串常量
    return "Hello, world!"
}