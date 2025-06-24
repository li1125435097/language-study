mod fb;
use fb::fibonacci_recursive;
use libloading::{Library, Symbol};
use std::time::SystemTime;

type FibSafe = unsafe extern "C" fn(u32) -> u64;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    
    unsafe {
        // 加载库
        let lib = Library::new("D:/codePlace/language-study/rust/fib-lib/target/release/fib_lib.dll")?;
        // 获取函数
        let fib_itera: Symbol<FibSafe> = lib.get(b"fib")?;
        let fib_rec: Symbol<FibSafe> = lib.get(b"fibonacci_recursive")?;
        // 调用函数
        let num = 37;
        let start = SystemTime::now();
        // let result = fib_itera(num);     // ddl迭代法调用
        // let result = fib_rec(num);          // ddl递归法调用
        let result = fibonacci_recursive(num);  // 模块递归法调用
        println!("耗时为：{:?}",start.elapsed().unwrap());
        println!("fbnq{}的值为：{}",num, result);
    }
    
    Ok(())
}
