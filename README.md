# bigfloat-bench

A benchmark of bigfloat libraries for Rust. This is reworked version of [bigint-benchmark-rs](https://github.com/tczajka/bigint-benchmark-rs).
Benchmark currently covers [rug](https://crates.io/crates/rug), [num-bigfloat](https://crates.io/crates/num-bigfloat), [astro-float](https://crates.io/crates/astro-float). It runs 5 iterations for each task and selects the best result. Each task uses an array of random normal floats with nearly the same precision.

## Usage

``` sh
cargo run --release -- --lib rug --lib num-bigfloat --lib astro-float -n 5 \
                       --task add_sub --task mul_div --task sqrt \
                       --task cbrt --task ln --task exp --task pow \
                       --task sin_asin --task cos_acos --task tan_atan \
                       --task sinh_asinh --task cosh_acosh --task tanh_atanh
```

## Results

| Task                                                                                   | rug (v1.16.0) | num-bigfloat (1.3.1) | astro-float (0.0.1) |
| --------------                                                                         | ------- | ------ | ------ |
|    add_sub (1,000,000 of additions and subtractions)                                   |  23 ms  |   176 ms  |    96 ms  |
|    mul_div (1,000,000 of multiplications and divisions)                                |  39 ms  |   231 ms  |   117 ms  |
|       sqrt (100,000 of square root computations)                                       |  17 ms  |   853 ms  |   417 ms  |
|       cbrt (100,000 of cube root computations)                                         |  56 ms  |  1347 ms  | not impl. |
|         ln (10,000 of natural logarith computations)                                   |  11 ms  |    59 ms  |   328 ms  |
|        exp (10,000 of e^n computations)                                                |  12 ms  |   134 ms  |    46 ms  |
|        pow (10,000 of power computations)                                              |  22 ms  |   144 ms  | not impl. |
|   sin_asin (10,000 of sine and then arcsine computations)                              |  61 ms  |   265 ms  |   673 ms  |
|   cos_acos (10,000 of cosine and then arccosine computations)                          |  59 ms  |   268 ms  |   754 ms  |
|   tan_atan (10,000 of tangent and then arctangent computations)                        |  58 ms  |   115 ms  |   563 ms  |
| sinh_asinh (10,000 of hyperbolic sine and then hyperbolic arcsine computations)        |  48 ms  |   276 ms  | not impl. |
| cosh_acosh (10,000 of hyperbolic cosine and then hyperbolic arccosine computations)    |  50 ms  |   281 ms  | not impl. |
| tanh_atanh (10,000 of hyperbolic tangent and then hyperbolic arctangent computations)  |  42 ms  |   203 ms  | not impl. |

Results for the precision 1,000 bit:

| Task                                                                                   | rug (v1.16.0) | astro-float (0.0.1) |
| --------------                                                                         | ------ | ------ |
|    add_sub (100,000 of additions and subtractions)                                   |   5 ms |  25 ms |
|    mul_div (100,000 of multiplications and divisions)                                |  19 ms |  66 ms |
|       sqrt (10,000 of square root computations)                                       |   8 ms | 101 ms |
|         ln (1,000 of natural logarith computations)                                   |   8 ms | 223 ms |
|        exp (1,000 of e^n computations)                                                |  13 ms |  27 ms |
|   sin_asin (1,000 of sine and then arcsine computations)                              |  61 ms | 500 ms |
|   cos_acos (1,000 of cosine and then arccosine computations)                          |  61 ms | 311 ms |
|   tan_atan (1,000 of tangent and then arctangent computations)                        |  35 ms | 279 ms |


Results for the precision 100,000 bits:


| Task                                                                                   | rug (v1.16.0) | astro-float (0.0.1) |
| --------------                                                                         | ------- | -------- |
|    add_sub (10,000 of additions and subtractions)                                      |   13 ms |    33 ms |
|    mul_div (100 of multiplications and divisions)                                      |   10 ms |    42 ms |
|       sqrt (100 of square root computations)                                           |   19 ms |   218 ms |
|         ln (10 of natural logarith computations)                                       |   76 ms |  8162 ms |
|        exp (10 of e^n computations)                                                    |  133 ms |   242 ms |
|   sin_asin (10 of sine and then arcsine computations)                                  |  435 ms |  9718 ms |
|   cos_acos (10 of cosine and then arccosine computations)                              |  438 ms | 10971 ms |
|   tan_atan (10 of tangent and then arctangent computations)                            |  429 ms | 11471 ms |
