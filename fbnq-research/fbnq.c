
#include <stdio.h>
#include <time.h>

int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n-1) + fibonacci(n-2);
}

int main() {
    clock_t start = clock();
    int result = fibonacci(37);
    double elapsed = (double)(clock() - start) / CLOCKS_PER_SEC * 1000;
    
    printf("斐波那契第37项: %d\n", result);
    printf("计算耗时: %.3f 毫秒\n", elapsed);
    return 0;
}
