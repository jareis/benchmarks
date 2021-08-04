const {
    performance
} = require('perf_hooks');


function fib(n) {
    if (n <= 0) {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

let eo = -performance.now();
fib(35);
eo += performance.now();
//console.log(`TIME: ${eo}ms`);
console.log(`TIME: ${eo * 0.001}s`)