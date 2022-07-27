use clap::{App, Arg};
use number::Number;
use std::time::{Duration, Instant};

mod number;
mod tasks;

fn main() {
    let args = App::new("Float numbers benchmarks")
        .arg(
            Arg::with_name("lib")
                .long("lib")
                .possible_values(&["rug", "num-bigfloat"])
                .multiple(true)
                .number_of_values(1)
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("task")
                .long("task")
                .possible_values(&["add_sub", "mul_div", "sqrt", "cbrt", "ln", "exp", "pow",
                    "sin_asin", "cos_acos", "tan_atan", "sinh_asinh", "cosh_acosh", "tanh_atanh"])
                .multiple(true)
                .number_of_values(1)
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("n")
                .short("n")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let libs: Vec<String> = args
        .values_of("lib")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    let tasks: Vec<String> = args
        .values_of("task")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    let n: usize = args.value_of("n").unwrap().parse().expect("invalid n");

    print!("{: >15}", " ");
    for lib in &libs {
        print!("{: >15}", lib);
    }
    println!();

    for task in &tasks {
        print!("{: >15}", task);
        for lib in &libs {
            let res = match lib.as_str() {
                "rug" => benchmark_lib_task::<rug::Float>(task, n),
                "num-bigfloat" => benchmark_lib_task::<num_bigfloat::BigFloat>(task, n),
                _ => unreachable!(),
            };
            print!("{: >15}", res);
        }
        println!();
    }
}

fn benchmark_lib_task<T: Number>(task: &str, n: usize) -> String {
    let vals: Vec<T> = get_range_for_task(task);
    let mut durations: Vec<u32> = Vec::new();
    for _ in 0..n {
        let mut full_dur = 0;
        let mut iter = 1;
        let mut niter = 0;
        while full_dur < 1000 && iter < 16 {
            niter += iter;
            for _ in 0..iter {
                let (_a, d) = run_task_using::<T>(task, &vals);
                full_dur += d.as_micros();
            }
            iter*=2;
        }
        durations.push((full_dur/niter/1000) as u32);
    }
    durations.sort_unstable();
    format!("{} ms", durations[0])
}

fn get_range_for_task<T: Number>(task: &str) -> Vec<T> {
    let (n, exp_range, exp_shift) = match task {
        "add_sub" => (1000000, 10, 40),
        "mul_div" => (1000000, 40, 40),
        "sqrt" => (100000, 256, 128),
        "cbrt" => (100000, 256, 128),
        "ln" => (10000, 256, 128),
        "exp" => (10000, 4, 40),
        "pow" => (10000, 4, 40),
        "sin_asin" => (10000, 3, 40),
        "cos_acos" => (10000, 3, 40),
        "tan_atan" => (10000, 3, 40),
        "sinh_asinh" => (10000, 3, 40),
        "cosh_acosh" => (10000, 3, 40),
        "tanh_atanh" => (10000, 3, 40),
        _ => unreachable!(),
    };
    T::rand_normal(n, exp_range, exp_shift)
}

fn run_task_using<T: Number>(task: &str, vals: &[T]) -> (T, Duration) {
    let start_time = Instant::now();

    let a = match task {
        "add_sub" => tasks::add_sub::<T>(vals),
        "mul_div" => tasks::mul_div::<T>(vals),
        "sqrt" => tasks::sqrt::<T>(vals),
        "cbrt" => tasks::cbrt::<T>(vals),
        "ln" => tasks::ln::<T>(vals),
        "exp" => tasks::exp::<T>(vals),
        "pow" => tasks::pow::<T>(&vals[10..], &vals[..10]),
        "sin_asin" => tasks::sin_asin::<T>(vals),
        "cos_acos" => tasks::cos_acos::<T>(vals),
        "tan_atan" => tasks::tan_atan::<T>(vals),
        "sinh_asinh" => tasks::sinh_asinh::<T>(vals),
        "cosh_acosh" => tasks::cosh_acosh::<T>(vals),
        "tanh_atanh" => tasks::tanh_atanh::<T>(vals),
        _ => unreachable!(),
    };

    let time = start_time.elapsed();
    (a, time)
}
