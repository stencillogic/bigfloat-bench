# bigfloat-bench

A benchmark of bigfloat libraries for Rust. 

This is reworked version of [bigint-benchmark-rs](https://github.com/tczajka/bigint-benchmark-rs).

Benchmark covers [rug](https://crates.io/crates/rug), [num-bigfloat](https://crates.io/crates/num-bigfloat), [astro-float](https://crates.io/crates/astro-float), [dashu-float](https://github.com/cmpute/dashu) (unfortunately, dashu-float doesn't work for some reason and is not present among the benchmark results at the moment, although you can try running benchmark with it included). 

Benchmark runs 5 iterations for each task and selects the best result. Each task uses an array of random normal floats with nearly the same precision.

## Usage

``` sh
cargo run --release -- --lib rug --lib num-bigfloat --lib astro-float -n 5 \
                       --task add --task sub --task mul --task div --task sqrt \
                       --task cbrt --task ln --task exp --task pow \
                       --task sin --task asin --task cos --task acos --task tan --task atan \
                       --task sinh --task asinh --task cosh --task acosh --task tanh --task atanh
```

## Results

Results for the precision 132 bit:

| Task                                          | rug (v1.16.0) | num-bigfloat (1.3.1) | astro-float (0.6.5) |
| --------------                                | ------- | ------ | ------ |
| 1,000,000 of additions                        |  45 ms  |   185 ms  |    104 ms  |
| 1,000,000 of subtractions                     |  42 ms  |   185 ms  |    103 ms  |
| 1,000,000 of multiplications                  |  34 ms  |   273 ms  |     97 ms  |
| 1,000,000 of divisions                        |  92 ms  |   630 ms  |    305 ms  |
| 100,000 of square root computations           |  16 ms  |   888 ms  |    803 ms  |
| 100,000 of cube root computations             |  54 ms  |  1359 ms  |   2320 ms  |
| 10,000 of natural logarith computations       |  22 ms  |    62 ms  |    527 ms  |
| 10,000 of e^n computations                    |  10 ms  |   116 ms  |    240 ms  |
| 10,000 of power computations                  |  25 ms  |   142 ms  |    487 ms  |
| 10,000 of sine computations                   |  12 ms  |    80 ms  |    145 ms  |
| 10,000 of cosine computations                 |  28 ms  |   119 ms  |    286 ms  |
| 10,000 of tangent computations                |   7 ms  |    78 ms  |    156 ms  |
| 10,000 of hyperbolic sine computations        |  29 ms  |   120 ms  |    362 ms  |
| 10,000 of hyperbolic cosine computations      |  15 ms  |    53 ms  |    330 ms  |
| 10,000 of hyperbolic tangent computations     |  22 ms  |    31 ms  |    131 ms  |
| 10,000 of arcsine computations                |  12 ms  |    61 ms  |    292 ms  |
| 10,000 of arccosine computations              |  72 ms  |    96 ms  |    746 ms  |
| 10,000 of arctangent computations             |  11 ms  |    57 ms  |    291 ms  |
| 10,000 of hyperbolic arcsine computations     |  27 ms  |   152 ms  |    722 ms  |
| 10,000 of hyperbolic arccosine computations   |  12 ms  |    69 ms  |    299 ms  |
| 10,000 of hyperbolic arctangent computations  |  28 ms  |    51 ms  |    587 ms  |

Results for the precision 1,000 bit:

| Task                                          | rug (v1.16.0) | astro-float (0.6.5) |
| --------------                                | ------- | ------ |
| 100,0000 of additions                         |   5 ms  |    15 ms  |
| 100,0000 of subtractions                      |   5 ms  |    15 ms  |
| 100,0000 of multiplications                   |  15 ms  |    34 ms  |
| 100,0000 of divisions                         |  26 ms  |   114 ms  |
| 10,000 of square root computations            |   4 ms  |   100 ms  |
| 10,000 of cube root computations              |  12 ms  |  1179 ms  |
| 1,000 of natural logarith computations        |  10 ms  |   154 ms  |
| 1,000 of e^n computations                     |   6 ms  |    73 ms  |
| 1,000 of power computations                   |   9 ms  |   137 ms  |
| 1,000 of sine computations                    |   6 ms  |    63 ms  |
| 1,000 of cosine computations                  |  14 ms  |    96 ms  |
| 1,000 of tangent computations                 |   4 ms  |    63 ms  |
| 1,000 of hyperbolic sine computations         |  14 ms  |   108 ms  |
| 1,000 of hyperbolic cosine computations       |   6 ms  |   135 ms  |
| 1,000 of hyperbolic tangent computations      |  13 ms  |    69 ms  |
| 1,000 of arcsine computations                 |   7 ms  |    85 ms  |
| 1,000 of arccosine computations               |  23 ms  |   197 ms  |
| 1,000 of arctangent computations              |   6 ms  |    86 ms  |
| 1,000 of hyperbolic arcsine computations      |  10 ms  |   188 ms  |
| 1,000 of hyperbolic arccosine computations    |   7 ms  |    88 ms  |
| 1,000 of hyperbolic arctangent computations   |  35 ms  |   153 ms  |

Results for the precision 10,000 bits:

| Task                                          | rug (v1.16.0) | astro-float (0.6.5) |
| --------------                                | ------- | ------ |
| 100,0000 of additions                         |   22 ms  |    50 ms  |
| 100,0000 of subtractions                      |   22 ms  |    51 ms  |
| 100,0000 of multiplications                   |  481 ms  |  1466 ms  |
| 100,0000 of divisions                         |  724 ms  |  4451 ms  |
| 10,000 of square root computations            |   49 ms  |   371 ms  |
| 1,000 of cube root computations               |   13 ms  |  4780 ms  |
| 100 of natural logarith computations          |   21 ms  |   769 ms  |
| 100 of e^n computations                       |   33 ms  |   448 ms  |
| 100 of power computations                     |   30 ms  |   644 ms  |
| 100 of sine computations                      |   37 ms  |   438 ms  |
| 100 of cosine computations                    |   35 ms  |   484 ms  |
| 100 of tangent computations                   |   35 ms  |   401 ms  |
| 100 of hyperbolic sine computations           |   36 ms  |   454 ms  |
| 100 of hyperbolic cosine computations         |   37 ms  |  1110 ms  |
| 100 of hyperbolic tangent computations        |   35 ms  |   426 ms  |
| 100 of arcsine computations                   |   34 ms  |   459 ms  |
| 100 of arccosine computations                 |   41 ms  |   813 ms  |
| 100 of arctangent computations                |   33 ms  |   456 ms  |
| 100 of hyperbolic arcsine computations        |   21 ms  |   774 ms  |
| 100 of hyperbolic arccosine computations      |   33 ms  |   456 ms  |
| 100 of hyperbolic arctangent computations     |   62 ms  |   704 ms  |
