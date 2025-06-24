use libc::{c_uint, c_ulonglong};
use std::mem;

// 导出C兼容接口
#[no_mangle]
pub extern "C" fn fib(n: c_uint) -> c_ulonglong {
    if n == 0 {
        return 0;
    }
    
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    b
}

#[no_mangle]
// 递归版本（不推荐，性能差）
pub extern "C" fn fibonacci_recursive(n: c_ulonglong) -> c_ulonglong {
    match n {
        0 | 1 => n,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

// 带错误处理的版本（返回结构体）
#[repr(C)]  // 确保C兼容内存布局
pub struct FibResult {
    value: c_ulonglong,
    error: c_uint, // 0=成功, 1=溢出
}

#[no_mangle]
pub extern "C" fn fib_safe(n: c_uint) -> FibResult {
    if n == 0 {
        return FibResult { value: 0, error: 0 };
    }

    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let next = a.checked_add(b);
        match next {
            Some(val) => {
                a = b;
                b = val;
            }
            None => {
                return FibResult {
                    value: u64::MAX,
                    error: 1,
                };
            }
        }
    }

    FibResult {
        value: b,
        error: 0,
    }
}

// 内存管理示例：分配斐波那契数列数组
#[no_mangle]
pub extern "C" fn fib_sequence(n: c_uint) -> *mut c_ulonglong {
    if n == 0 {
        return std::ptr::null_mut();
    }

    let mut vec = Vec::with_capacity(n as usize);
    vec.push(0);
    if n > 1 {
        vec.push(1);
        for i in 2..n {
            vec.push(vec[(i - 1) as usize] + vec[(i - 2) as usize]);
        }
    }

    // 转移所有权给调用方
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);  // 防止Rust释放内存
    ptr
}

// 释放内存的C接口
#[no_mangle]
pub extern "C" fn free_fib_sequence(ptr: *mut c_ulonglong, len: c_uint) {
    if !ptr.is_null() {
        unsafe {
            // 重新构建Vec并立即丢弃
            let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
        }
    }
}