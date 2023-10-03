# Rust f32 Crunching

This is a simple benchmark to compare the performance of crunching f32s in Rust. I wanted
to test the speed of single vs. multithreaded operations, and how they scale with more data
and more threads.

## Results

Normal Test:

```rs
Crunching 1,000,000 numbers...
Single-threaded time: 858.352µs
Multi-threaded time: 3.667061ms
Single-threaded is 76.59% faster than multi-threaded.
------------------------------------------------------------
Crunching 10,000,000 numbers...
Single-threaded time: 8.145755ms
Multi-threaded time: 5.620677ms
Multi-threaded is 31.00% faster than single-threaded.
------------------------------------------------------------
Crunching 100,000,000 numbers...
Single-threaded time: 89.469167ms
Multi-threaded time: 41.795485ms
Multi-threaded is 53.29% faster than single-threaded.
------------------------------------------------------------
Crunching 1,000,000,000 numbers...
Single-threaded time: 912.151216ms
Multi-threaded time: 371.779317ms
Multi-threaded is 59.24% faster than single-threaded.
------------------------------------------------------------
```

Vectorized Test:
```rs
Vector 1 size 1,000,000 Single-threaded time: 366.938µs
Vector 1 size 1,000,000 Multi-threaded time: 3.733378ms
Single-threaded is 90.17% faster than multi-threaded.
------------------------------------------------------------
Vector 2 size 10,000,000 Single-threaded time: 5.808091ms
Vector 2 size 10,000,000 Multi-threaded time: 8.105488ms
Single-threaded is 28.34% faster than multi-threaded.
------------------------------------------------------------
Vector 3 size 100,000,000 Single-threaded time: 60.996867ms
Vector 3 size 100,000,000 Multi-threaded time: 98.880694ms
Single-threaded is 38.31% faster than multi-threaded.
------------------------------------------------------------
Vector 4 size 1,000,000,000 Single-threaded time: 618.643122ms
Vector 4 size 1,000,000,000 Multi-threaded time: 907.586136ms
Single-threaded is 31.84% faster than multi-threaded.
------------------------------------------------------------
```

_This is ran on my Ryzen 5 2600 with 6 cores and 12 threads. Performance may be different on your system..._

## Running the benchmark

To run simply run the [test.sh](./test.sh) script. It will compile the code and run the benchmark.

Runs the benchmark.
```shell
$ ./test.sh <normal|vec|all>
````
Runs the vectorized benchmark.
