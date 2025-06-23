# rust命令使用

## rustc命令使用
- 生成可执行文件
rustc hellow.rs -o hellow
- 生成静态库
rustc --crate-type=lib hellow.rs
- 生成动态库
rustc --crate-type=dylib hellow.rs
- 生成文档
rustdoc hellow.rs
- 生成测试
rustc --test hellow.rs
- 生成检查
rustc --crate-type=bin hellow.rs

## cargo命令使用
- 创建项目
cargo new hellow
- 添加依赖
cargo add serde
- 移除依赖
cargo remove serde
- 更新依赖
cargo update
- 初始化项目
cargo init
- 构建项目
cargo build release
- 运行项目
cargo run
- 测试项目
cargo test
- 检查项目
cargo check
- 清理项目
cargo clean
- 生成文档
cargo doc
- 发布项目
cargo publish