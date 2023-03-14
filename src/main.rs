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
                .possible_values(&["add", "sub", "mul", "div", "sqrt", "cbrt", "ln", "exp", "pow",
                    "sin", "asin", "cos", "acos", "tan", "atan", "sinh", "asinh", "cosh", "acosh", "tanh", "atanh"])
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
    // exponent has base 10
    let (n, exp_from, exp_to, sign_positive) = match task {
        "add" => (1000000, -10, 10, false),
        "sub" => (1000000, -10, 10, false),
        "mul" => (1000000, -10, 10, false),
        "div" => (1000000, -10, 10, false),
        "sqrt" => (100000, -10, 10, true),
        "cbrt" => (100000, -10, 10, false),
        "ln" => (10000, -10, 10, true),
        "exp" => (10000, -10, 3, false),
        "pow" => (10000, -5, 5, false),
        "sin" => (10000, -10, 3, false),
        "cos" => (10000, -10, 3, false),
        "tan" => (10000, -10, 3, false),
        "sinh" => (10000, -10, 3, false),
        "cosh" => (10000, -10, 3, false),
        "tanh" => (10000, -10, 3, false),
        "asin" => (10000, -10, 0, false),
        "acos" => (10000, -10, 0, false),
        "atan" => (10000, -10, 0, false),
        "asinh" => (10000, -10, 10, false),
        "acosh" => (10000, 1, 10, true),
        "atanh" => (10000, -10, 0, false),
        _ => unreachable!(),
    };
    T::rand_normal(n, exp_from, exp_to, gs, sign_positive)
}

fn run_task_using<G: GlobalState, T: Number<G>>(task: &str, vals: &[T]) -> (T, Duration) {
    let start_time = Instant::now();

    let a = match task {
        "add" => tasks::task_for_two_args::<G, T>(vals, T::add),
        "sub" => tasks::task_for_two_args::<G, T>(vals, T::sub),
        "mul" => tasks::task_for_two_args::<G, T>(vals, T::mul),
        "div" => tasks::task_for_two_args::<G, T>(vals, T::div),
        "sqrt" => tasks::task_for_one_arg::<G, T>(vals, T::sqrt),
        "cbrt" => tasks::task_for_one_arg::<G, T>(vals, T::cbrt),
        "ln" => tasks::task_for_one_arg::<G, T>(vals, T::ln),
        "exp" => tasks::task_for_one_arg::<G, T>(vals, T::exp),
        "pow" => tasks::task_for_two_args::<G, T>(vals, T::pow),
        "sin" => tasks::task_for_one_arg::<G, T>(vals, T::sin),
        "cos" => tasks::task_for_one_arg::<G, T>(vals, T::cos),
        "tan" => tasks::task_for_one_arg::<G, T>(vals, T::tan),
        "sinh" => tasks::task_for_one_arg::<G, T>(vals, T::sinh),
        "cosh" => tasks::task_for_one_arg::<G, T>(vals, T::cosh),
        "tanh" => tasks::task_for_one_arg::<G, T>(vals, T::tanh),
        "asin" => tasks::task_for_one_arg::<G, T>(vals, T::asin),
        "acos" => tasks::task_for_one_arg::<G, T>(vals, T::acos),
        "atan" => tasks::task_for_one_arg::<G, T>(vals, T::atan),
        "asinh" => tasks::task_for_one_arg::<G, T>(vals, T::asinh),
        "acosh" => tasks::task_for_one_arg::<G, T>(vals, T::acosh),
        "atanh" => tasks::task_for_one_arg::<G, T>(vals, T::atanh),
        _ => unreachable!(),
    };

    let time = start_time.elapsed();
    (a, time)
}
