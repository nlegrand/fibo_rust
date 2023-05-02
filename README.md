A fibo nth number calculator made from the suggestion at the end of
the Rust Book chapter 3.

```
$ cd fibo
$ cargo run
   Compiling fibo v0.1.0 (/Users/nicolas/Repositories/rust_exos/fibo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/fibo`
Please type the nth Fibn number to calculate:
19
fibo 1: 1
fibo 2: 1
fibo 3: 2
fibo 4: 3
fibo 5: 5
fibo 6: 8
fibo 7: 13
fibo 8: 21
fibo 9: 34
fibo 10: 55
fibo 11: 89
fibo 12: 144
fibo 13: 233
fibo 14: 377
fibo 15: 610
fibo 16: 987
fibo 17: 1597
fibo 18: 2584
fibo 19: 4181
Please type the nth Fibn number to calculate:
92
Sorry, 92 would overflow the i64 type, 91 is the limit
Please type the nth Fibn number to calculate:

```