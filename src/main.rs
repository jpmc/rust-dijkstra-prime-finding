mod impls;
use crate::impls::{dijkstra_naive, dijkstra_attempt_1, dijkstra_attempt_2, dijkstra_attempt_3};

use core::time::Duration;

// @TODO Update README with any relevant build info for new 4-binary strategy.
// @TODO Look into further benchmarking to measure runtime and peak memory usage.

// Inspiration: https://youtu.be/fwxjMKBMR7s
fn main() {

    // Benchmark parameters. Iterations per Input size, List of Input sizes to test.
    const ITERATIONS: u32 = 100;
    const INPUTS: [u64; 2] = [10_000, 100_000];//, 1_000_000];
    // Expects: 4, 25, 168,

    // Array of functions to test. Currently using separate sets for separate tuple-size return types.
    // @TODO Find a way to better "generically" use these function pointers in one test set.
    const FUNCS_2PLE: [(&str, fn(u64) ->Vec<(u64, u64)>);3] = [
        ("naive", dijkstra_naive::run),
        ("attempt 1", dijkstra_attempt_1::run),
        ("attempt 3", dijkstra_attempt_3::run),
    ];
    const FUNCS_3PLE: [(&str, fn(u64) ->Vec<(u64, u64, u64)>);1] = [
        ("attempt 2", dijkstra_attempt_2::run)
    ];

    // Run the tests.
    for f_in in INPUTS {
        for (name, f) in FUNCS_2PLE {
            println!("Running \"{}\": N = {}, {} iterations", name, f_in, ITERATIONS);
            let results = benchmark_run_2(&f, f_in, ITERATIONS);
            println!("{:#?}", results);
        }
        for (name, f) in FUNCS_3PLE {
            println!("Running \"{}\": N = {}, {} iterations", name, f_in, ITERATIONS);
            let results = benchmark_run_3(&f, f_in, ITERATIONS);
            println!("{:#?}", results);
        }
    }
}

#[derive(Debug)]
struct RunResults {
    total_elapsed_time_user: Duration,
    avg_elapsed_time_user: Duration,
    // Bytes is the only one where having a total doesn't make sense.
    //avg_bytes_used: u64,
    // Just keep a count of how many primes are found in an iteration.
    prime_amount: usize,
}
#[allow(unused_assignments)]
fn benchmark_run_2(func: &dyn Fn(u64) -> Vec<(u64, u64)>, input: u64, iterations: u32) -> RunResults {
    // Prep empty Run Results.
    let mut results = RunResults {
        total_elapsed_time_user: Duration::ZERO,
        avg_elapsed_time_user: Duration::ZERO,
        //avg_bytes_used: 0,
        prime_amount: 0,
    };

    // Set up iterations loop.
    let mut counter = iterations.clone();
    loop {
        // 1. Get initial runtimes and memory to diff against after running.
        let mut primes:Vec<(u64, u64)> = Vec::new();
        let start = std::time::Instant::now();
        // 2. Run the algorithm.
        primes = func(input);
        // 3.  Diff against post-run system state, then add to running results.
        // 3a. Averages are the same as Totals until they are divided after all iterations end.
        let end = start.elapsed();
        results.total_elapsed_time_user += end;
        //results.avg_bytes_used += after.memory_usage_bytes - initial.memory_usage_bytes;
        results.prime_amount = primes.len();
        // 4. Initial & After stats overwritten, but flush primes vector manually to free memory between runs.
        //    Still not working?
        primes.clear();
        primes.shrink_to(0);
        // 5. Decrement-and-check iteration counter.
        counter -= 1;
        if counter <= 0 {
            break;
        }
    }

    // Average all the proper Results fields.
    //results.avg_bytes_used = results.avg_bytes_used / (iterations as u64);
    results.avg_elapsed_time_user = results.total_elapsed_time_user / iterations;

    results
}

#[allow(unused_assignments)]
fn benchmark_run_3(func: &dyn Fn(u64) -> Vec<(u64, u64, u64)>, input: u64, iterations: u32) -> RunResults {
    // Prep empty Run Results.
    let mut results = RunResults {
        total_elapsed_time_user: Duration::ZERO,
        avg_elapsed_time_user: Duration::ZERO,
        //avg_bytes_used: 0,
        prime_amount: 0,
    };

    // Set up iterations loop.
    let mut counter = iterations.clone();
    loop {
        // 1. Get initial runtimes and memory to diff against after running.
        let mut primes:Vec<(u64, u64, u64)> = Vec::new();
        let start = std::time::Instant::now();
        // 2. Run the algorithm.
        primes = func(input);
        // 3.  Diff against post-run system state, then add to running results.
        // 3a. Averages are the same as Totals until they are divided after all iterations end.
        let end = start.elapsed();
        results.total_elapsed_time_user += end;
        //results.avg_bytes_used += after.memory_usage_bytes - initial.memory_usage_bytes;
        results.prime_amount = primes.len();
        // 4. Initial & After stats overwritten, but flush primes vector manually to free memory between runs.
        //    Still not working?
        primes.clear();
        primes.shrink_to(0);
        // 5. Decrement-and-check iteration counter.
        counter -= 1;
        if counter <= 0 {
            break;
        }
    }

    // Average all the proper Results fields.
    //results.avg_bytes_used = results.avg_bytes_used / (iterations as u64);
    results.avg_elapsed_time_user = results.total_elapsed_time_user / iterations;

    results
}
