# Rust f32 Crunching

This is a simple benchmark to compare the performance of crunching f32s in Rust. I wanted
to test the speed of single vs multithreaded operations, and how they scale with more data
and more threads.

## Results

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

*This is ran on my Ryzen 5 2600 with 6 cores and 12 threads. Performance may be different on your system...*

# Running the benchmark

To run simply run the [test.sh](./test.sh) script. It will compile the code and run the benchmark.