use std::env;

#[allow(dead_code)]
fn main() {
    // Allocate input argument.
    let upto: u64;
    let args: Vec<String> = env::args_os().collect();
    if args.len() == 2 {
        upto = match args[1].parse() {
            Ok(n) => n,
            Err(_) => 100,
        }
    } else {
        upto = 100;
    }

    let primes = run(upto);

    println!("{}", primes.iter()
        .map(|t| t.0.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
}

// Attempt 1: Add global sqrt boundary to speed up iterations.
pub fn run(upto: u64) -> Vec<(u64, u64)> {
    // !! Addition add sqrt of sample size as a limiter of sorts for later.
    // !! It's a 'global' sqrt boundary rather than individualized by int per loop.
    let sqrt: u64 = (upto as f64).sqrt().ceil() as u64;

    let size: usize = prime_count_upper_bound(upto) as usize;
    let mut pool: Vec<(u64, u64)> = Vec::with_capacity(size).clone();
    pool.resize(size, (0, 0));
    pool[0] = (2, 4);

    let mut index = 1;

    // Begin running the algorithm and populate with more primes.
    for n in 3..=upto {
        let mut add_to_pool: bool = true;
        for tuple in &mut pool {
            // !! Addition of the 'global' sqrt boundary.
            if tuple.0 == 0 || tuple.0 > sqrt{
                break;
            }
            if n >= tuple.1 {
                add_to_pool = false;
                tuple.1 += tuple.0;
            }
        }

        if add_to_pool {
            pool[index] = (n, n * n);
            index += 1;
        }
    }

    for _ in index..pool.len() {
        pool.pop();
    }

    pool
}

// Max # of primes that can exist below x:
//      pi(x) <= 1.25066 * (x / ln(x))
// Sources:
// - https://math.stackexchange.com/a/16090/184655
// - https://en.wikipedia.org/wiki/Prime-counting_function#Inequalities
fn prime_count_upper_bound(upto: u64) -> u64 {
    let f_upto: f64 = upto as f64;
    (1.25066 * (f_upto / f_upto.ln())).ceil() as u64
}