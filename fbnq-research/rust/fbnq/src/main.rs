use std::time::Instant;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let start = Instant::now();
    let result = fibonacci(37);
    let duration = start.elapsed();
    
    println!("斐波那契数列第37项: {}", result);
    println!("耗时: {:?}", duration);
}
