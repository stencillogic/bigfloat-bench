# bigfloat-bench

A benchmark of bigfloat libraries for Rust. 

This is reworked version of [bigint-benchmark-rs](https://github.com/tczajka/bigint-benchmark-rs).

Benchmark covers [rug](https://crates.io/crates/rug), [num-bigfloat](https://crates.io/crates/num-bigfloat), [astro-float](https://crates.io/crates/astro-float), [dashu-float](https://github.com/cmpute/dashu) (unfortunately, dashu-float doesn't work for some reason and is not present among the benchmark results at the moment, although you can try running benchmark with it included). 

Benchmark runs 5 iterations for each task and selects the best result. Each task uses an array of random normal floats with nearly the same precision.

## Usage

``` sh
cargo run --release -- --lib rug --lib num-bigfloat --lib astro-float -n 5 \
                       --task add_sub --task mul_div --task sqrt \
                       --task cbrt --task ln --task exp --task pow \
                       --task sin_asin --task cos_acos --task tan_atan \
                       --task sinh_asinh --task cosh_acosh --task tanh_atanh
```

## Results

Results for the precision 132 bit:

| Task                                                                                   | rug (v1.16.0) | num-bigfloat (1.3.1) | astro-float (0.3.0) |
| --------------                                                                         | ------- | ------ | ------ |
|    add_sub (1,000,000 of additions and subtractions)                                   |  23 ms  |   176 ms  |   87 ms  |
|    mul_div (1,000,000 of multiplications and divisions)                                |  39 ms  |   231 ms  |  122 ms  |
|       sqrt (100,000 of square root computations)                                       |  17 ms  |   853 ms  |  385 ms  |
|       cbrt (100,000 of cube root computations)                                         |  56 ms  |  1347 ms  |  778 ms  |
|         ln (10,000 of natural logarith computations)                                   |  11 ms  |    59 ms  |  308 ms  |
|        exp (10,000 of e^n computations)                                                |  12 ms  |   134 ms  |  212 ms  |
|        pow (10,000 of power computations)                                              |  22 ms  |   144 ms  |  596 ms  |
|   sin_asin (10,000 of sine and then arcsine computations)                              |  61 ms  |   265 ms  |  642 ms  |
|   cos_acos (10,000 of cosine and then arccosine computations)                          |  59 ms  |   268 ms  |  723 ms  |
|   tan_atan (10,000 of tangent and then arctangent computations)                        |  58 ms  |   115 ms  |  772 ms  |
| sinh_asinh (10,000 of hyperbolic sine and then hyperbolic arcsine computations)        |  48 ms  |   276 ms  |  609 ms  |
| cosh_acosh (10,000 of hyperbolic cosine and then hyperbolic arccosine computations)    |  50 ms  |   281 ms  |  610 ms  |
| tanh_atanh (10,000 of hyperbolic tangent and then hyperbolic arctangent computations)  |  42 ms  |   203 ms  |  618 ms  |

Results for the precision 1,000 bit:

| Task                                                                                   | rug (v1.16.0) | astro-float (0.3.0) |
| --------------                                                                         | ------ | ------ |
|    add_sub (100,000 of additions and subtractions)                                     |   3 ms  |   13 ms  |
|    mul_div (100,000 of multiplications and divisions)                                  |  10 ms  |   39 ms  |
|       sqrt (10,000 of square root computations)                                        |   4 ms  |   62 ms  |
|       cbrt (10,000 of cube root computations)                                          |  11 ms  |  259 ms  |
|         ln (1,000 of natural logarith computations)                                    |   5 ms  |  126 ms  |
|        exp (1,000 of e^n computations)                                                 |   7 ms  |   74 ms  |
|        pow (1,000 of power computations)                                               |   9 ms  |  213 ms  |
|   sin_asin (1,000 of sine and then arcsine computations)                               |  37 ms  |  254 ms  |
|   cos_acos (1,000 of cosine and then arccosine computations)                           |  36 ms  |  271 ms  |
|   tan_atan (1,000 of tangent and then arctangent computations)                         |  32 ms  |  314 ms  |
| sinh_asinh (1,000 of hyperbolic sine and then hyperbolic arcsine computations)         |  19 ms  |  221 ms  |
| cosh_acosh (1,000 of hyperbolic cosine and then hyperbolic arccosine computations)     |  20 ms  |  221 ms  |
| tanh_atanh (1,000 of hyperbolic tangent and then hyperbolic arctangent computations)   |  19 ms  |  246 ms  |

Results for the precision 100,000 bits:

| Task                                                                                   | rug (v1.16.0) | astro-float (0.3.0) |
| --------------                                                                         | ------- | -------- |
|    add_sub (10,000 of additions and subtractions)                                      |   13 ms  |     35 ms  |
|    mul_div (100 of multiplications and divisions)                                      |   10 ms  |     42 ms  |
|       sqrt (100 of square root computations)                                           |   19 ms  |    397 ms  |
|       cbrt (10 of cube root computations)                                              |    4 ms  |    420 ms  |
|         ln (10 of natural logarith computations)                                       |   76 ms  |   8650 ms  |
|        exp (10 of e^n computations)                                                    |  134 ms  |   4971 ms  |
|        pow (10 of power computations)                                                  |   93 ms  |  13493 ms  |
|   sin_asin (10 of sine and then arcsine computations)                                  |  434 ms  |  10533 ms  |
|   cos_acos (10 of cosine and then arccosine computations)                              |  431 ms  |   9983 ms  |
|   tan_atan (10 of tangent and then arctangent computations)                            |  427 ms  |  18468 ms  |
| sinh_asinh (10 of hyperbolic sine and then hyperbolic arcsine computations)            |  235 ms  |  13616 ms  |
| cosh_acosh (10 of hyperbolic cosine and then hyperbolic arccosine computations)        |  237 ms  |  13668 ms  |
| tanh_atanh (10 of hyperbolic tangent and then hyperbolic arctangent computations)      |  239 ms  |  17406 ms  |
