#include <iostream>
#include <chrono>

using namespace std;
using namespace std::chrono;

long long fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n-1) + fibonacci(n-2);
}

int main() {
    int n = 37;
    auto start = high_resolution_clock::now();
    long long result = fibonacci(n);
    auto stop = high_resolution_clock::now();
    auto duration = duration_cast<milliseconds>(stop - start);
    
    cout << "斐波那契数列第" << n << "项: " << result << endl;
    cout << "计算耗时: " << duration.count() << " 毫秒" << endl;
    
    return 0;
}
