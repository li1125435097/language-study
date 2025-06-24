pub fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}