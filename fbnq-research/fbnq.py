
import time

def fibonacci(n):
    if n <= 1:
        return n
    else:
        return fibonacci(n-1) + fibonacci(n-2)

start_time = time.time()
result = fibonacci(37)
end_time = time.time()

print(f"斐波那契数列第37项: {result}")
print(f"计算耗时: {end_time - start_time:.4f}秒")
