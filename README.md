# bigfloat-bench

A benchmark of bigfloat libraries for Rust. This is reworked version of [bigint-benchmark-rs](https://github.com/tczajka/bigint-benchmark-rs).
Benchmark currently covers [rug](https://crates.io/crates/rug) and [num-bigfloat](https://crates.io/crates/num-bigfloat). It runs 5 iterations for each task and selects the best result. Each task uses an array of random noraml floats with nearly the same precision.

## Usage

``` sh
cargo run --release -- --lib rug --lib num-bigfloat -n 5 \
                       --task add_sub --task mul_div --task sqrt \
                       --task cbrt --task ln --task exp --task pow \
                       --task sin_asin --task cos_acos --task tan_atan \
                       --task sinh_asinh --task cosh_acosh --task tanh_atanh
```

## Results

| Task                                                                                  | rug (v1.16.0) | num-bigfloat(1.3.1) |
| --------------                                                                        | ------- | ------ |
|    add_sub (1,000,000 of additions and subtractions)                                    |  23 ms  |   176 ms  |
|    mul_div (1,000,000 of multiplications and divisions)                                 |  39 ms  |   231 ms  |
|       sqrt (100,000 of square root computations)                                       |  17 ms  |   853 ms  |
|       cbrt (100,000 of cube root computations)                                         |  56 ms  |  1347 ms  |
|         ln (10,000 of natural logarith computations)                                   |  11 ms  |    59 ms  |
|        exp (10,000 of e^n computations)                                                |  12 ms  |   134 ms  |
|        pow (10,000 of power computations)                                              |  22 ms  |   144 ms  |
|   sin_asin (10,000 of sine and then arcsine computations)                              |  61 ms  |   265 ms  |
|   cos_acos (10,000 of cosine and then arccosine computations)                          |  59 ms  |   268 ms  |
|   tan_atan (10,000 of tangent and then arctangent computations)                        |  58 ms  |   115 ms  |
| sinh_asinh (10,000 of hyperbolic sine and then hyperbolic arcsine computations)        |  48 ms  |   276 ms  |
| cosh_acosh (10,000 of hyperbolic cosine and then hyperbolic arccosine computations)    |  50 ms  |   281 ms  |
| tanh_atanh (10,000 of hyperbolic tangent and then hyperbolic arctangent computations)  |  42 ms  |   203 ms  |
