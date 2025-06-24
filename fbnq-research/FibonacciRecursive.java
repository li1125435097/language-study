
public class FibonacciRecursive {
    public static int fibonacci(int n) {
        if (n <= 1) {
            return n;
        }
        return fibonacci(n-1) + fibonacci(n-2);
    }

    public static void main(String[] args) {
        long startTime = System.nanoTime();
        int result = fibonacci(37);
        long endTime = System.nanoTime();
        
        System.out.println("result " + result);
        System.out.println("time " + (endTime - startTime)/1000000 + "ms");
    }
}
