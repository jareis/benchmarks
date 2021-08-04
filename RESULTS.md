PHP 7.3.9

> time php -n fib.php
> TIME: 9.052820268E+18s
> real 0m9.167s
> user 0m9.039s
> sys 0m0.025s

PHP 8.0.1

> time php -n fib.php
> TIME: 2.311611393E+18s
> real 0m2.626s
> user 0m2.328s
> sys 0m0.039s

Rust

> time ./fib
> Time: 0
> real 0m0.144s
> user 0m0.139s
> sys 0m0.003s

golang

> time ./fib
> TIME: 0.07s
> real 0m3.790s
> user 0m0.070s
> sys 0m0.004s

node v15.7.0

> time node fib.js
> TIME: 0.218507
> real 0m0.291s
> user 0m0.267s
> sys 0m0.016s

deno 1.7.1

> time deno run fib.ts
> TIME: 0.26s
> real 0m0.834s
> user 0m0.958s
> sys 0m0.062s

Python 3.9

> time python3.9 fib.py
> TIME: 4.998s
> real 0m5.096s
> user 0m5.028s
> sys 0m0.029s
