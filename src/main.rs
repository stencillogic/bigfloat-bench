use clap::{App, Arg};
use dashu_float::{FBig, round::mode::HalfEven};
use number::{Number, GlobalState};
use std::time::{Duration, Instant};

use crate::number::{StubGlobalState, AstroGlobalState};

mod number;
mod tasks;
mod astro;

fn main() {
    let args = App::new("Float numbers benchmarks")
        .arg(
            Arg::with_name("lib")
                .long("lib")
                .possible_values(&["rug", "num-bigfloat", "dashu-float", "astro-float"])
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
                "rug" => benchmark_lib_task::<StubGlobalState, rug::Float>(task, n),
                "num-bigfloat" => benchmark_lib_task::<StubGlobalState, num_bigfloat::BigFloat>(task, n),
                "dashu-float" => benchmark_lib_task::<StubGlobalState, FBig<HalfEven, 2>>(task, n),
                "astro-float" => benchmark_lib_task::<AstroGlobalState, crate::astro::AstroFloat>(task, n),
                _ => unreachable!(),
            };
            print!("{: >15}", res);
        }
        println!();
    }
}

fn benchmark_lib_task<G: GlobalState, T: Number<G>>(task: &str, n: usize) -> String {
    let vals: Vec<T> = get_range_for_task(task);
    let mut durations: Vec<u32> = Vec::new();
    for _ in 0..n {
        let mut full_dur = 0;
        let mut iter = 1;
        let mut niter = 0;
        while full_dur < 1000 && iter < 16 {
            niter += iter;
            for _ in 0..iter {
                let (_a, d) = run_task_using::<G, T>(task, &vals);
                full_dur += d.as_micros();
            }
            iter*=2;
        }
        durations.push((full_dur/niter/1000) as u32);
    }
    durations.sort_unstable();
    format!("{} ms", durations[0])
}

fn get_range_for_task<G: GlobalState, T: Number<G>>(task: &str) -> Vec<T> {
    let gs = T::global_state();
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
    T::rand_normal(n, exp_range, exp_shift, gs)
}

fn run_task_using<G: GlobalState, T: Number<G>>(task: &str, vals: &[T]) -> (T, Duration) {
    let start_time = Instant::now();

    let a = match task {
        "add_sub" => tasks::add_sub::<G, T>(vals),
        "mul_div" => tasks::mul_div::<G, T>(vals),
        "sqrt" => tasks::sqrt::<G, T>(vals),
        "cbrt" => tasks::cbrt::<G, T>(vals),
        "ln" => tasks::ln::<G, T>(vals),
        "exp" => tasks::exp::<G, T>(vals),
        "pow" => tasks::pow::<G, T>(&vals[10..], &vals[..10]),
        "sin_asin" => tasks::sin_asin::<G, T>(vals),
        "cos_acos" => tasks::cos_acos::<G, T>(vals),
        "tan_atan" => tasks::tan_atan::<G, T>(vals),
        "sinh_asinh" => tasks::sinh_asinh::<G, T>(vals),
        "cosh_acosh" => tasks::cosh_acosh::<G, T>(vals),
        "tanh_atanh" => tasks::tanh_atanh::<G, T>(vals),
        _ => unreachable!(),
    };

    let time = start_time.elapsed();
    (a, time)
}
