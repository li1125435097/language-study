mod son;
mod orm;
mod concurrency;
mod nestmod;
mod mmacro;

use std::rc::Rc;

use nestmod::db;
use son::add::add;
use son::file;
use orm::Orm;
fn main() {
    println!("{}",add(1, 2));

    // 模块封装引用
    let handle = Orm::Handle::new(100110,false);
    handle.create(12);
    handle.del(12);
    handle.update(12);
    handle.read(12);
    
    // 嵌套模块封装引用
    db::handle::hand(123);

    // 报错
    // panic!("error");

    // 文件操作
    let filename = "D:/codePlace/language-study/rust/advance-grammer/src/test.txt";

    // 文件读取
    let str = file::getfile(filename);
    println!("{}",str);
    // 二进制读取
    let buf = file::getfileBuf(filename);
    println!("{:?}",buf);
    // 文件流读取
    let mut buffer = [0u8; 5];
    file::getfileChunk(filename, &mut buffer);
    println!("{:?}",buffer);
    
    // 文件写入
    file::writeFile("ab.txt", "锄禾日当午，汗滴禾下土。谁知盘中餐，粒粒皆辛苦。");
    file::coverFile("ab.txt", "春风又绿江南岸，明月何时照我还。");
    
    // 向量操作
    let mut vec:Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}",vec);

    let mut v = vec![1,2,3,4,5];
    v.push(6);
    println!("{:?}",v);
    v.pop();
    println!("{:?}",v);

    // 并发
    // concurrency::start(Some(10000));

    // 宏
    greet!("world");

    // 智能指针
    // Box<T> 是 Rust 中最简单的智能指针之一，它允许在堆上分配一块内存，并将值存储在这个内存中。可被多次引用
    // 由于 Rust 的所有权规则，使用 Box 可以在堆上创建具有已知大小的数据。
    let bptr = Box::new(String::from("hello"));
    println!("{}",bptr);
    let baptr = &bptr;
    let bbptr = &bptr;
    println!("{:?}",baptr.as_ptr());
    println!("{:?}",bptr.as_ptr());
    println!("{:?}",bbptr.as_ptr());

    // Arc<T>（原子引用计数指针）与 Rc<T> 类似，但是可以安全地在多线程环境中共享数据，因为它使用原子操作来更新引用计数    
    let rptr = Rc::new(0);

}   
