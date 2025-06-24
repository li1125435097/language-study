const ffi = require('ffi');

const lib = ffi.Library("D:/codePlace/language-study/rust/fib-lib/target/release/fib_lib.dll", {
  'fibonacci_recursive': ['int', ['int']]
});

const result = lib.fibonacci_recursive(37);
console.log('Result:', result); // 输出: Result: 5
