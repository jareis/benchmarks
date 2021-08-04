<?php

function fib(int $n): int
{
    if ($n < 1) {
        return 1;
    }

    return fib($n - 1) + fib($n - 2);
}

$eot = -hrtime(true);
fib(35);
$eot += hrtime(true);

printf("TIME: %ss", $eot/1e-9);
