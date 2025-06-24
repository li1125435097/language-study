// 运行脚本app.js
const {fibonacci} = require('./build/Release/fibonacci');
// 测试计算
console.time()
let a = fibonacci(37)
console.timeEnd()
