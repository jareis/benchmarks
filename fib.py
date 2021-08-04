import time

def fib(n):
  if n <= 1:
    return 1
  return fib(n - 1) + fib(n - 2)

duration = -time.time()
fib(35)
duration += time.time()
print(f"{duration} s")