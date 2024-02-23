use std::env;

#[allow(dead_code)]
fn main() {
    // Allocate input argument.
    let upto: u64;
    // Note, using args_os for OsStrings that do not panic when invalid unicode is passed.
    // Src: https://stackoverflow.com/a/15621897/4147290
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

// First working implementation (or Attempt 0)
pub fn run(upto: u64) -> Vec<(u64, u64)> {
    // Calculate the most primes that can exist between 2 and `upto`.
    // Then preallocate a Vector with that capacity if empty tuples.
    // Note: Can't index into the Vec as it's capacity/memory chunk is set, but it has a size of 0.
    //  See: https://stackoverflow.com/a/27176996/4147290
    let size: usize = prime_count_upper_bound(upto) as usize;
    let mut pool: Vec<(u64, u64)> = Vec::with_capacity(size).clone();
    pool.resize(size, (0, 0));

    // Seed the pool with the first prime and it's square.
    pool[0] = (2, 4);

    // Index counter to keep track of the tail of the primes, the last non-zero tuple to insert into.
    // 0 was already populated with (2, 4) above.
    let mut index = 1;

    // Begin running the algorithm and populate with more primes.
    for n in 3..=upto {
        // Iterate through the Pool, and check for if it is smaller than the smallest multiple(s).
        let mut add_to_pool: bool = true;
        for tuple in &mut pool {

            // Stop at the first empty tuple in the pool.
            if tuple.0 == 0 {
                break;
            }
            // For each existing tuple, check if `n` is less than any multiples in the pool.
            if n >= tuple.1 {
                // If it is a composite, mark as not to be added to the prime pool.
                // Then increment all multiples < `n` by their respective primes.
                add_to_pool = false;
                tuple.1 += tuple.0;
            }
        }

        // If a candidate `n` qualified for the pool, add it with its square as an entry, then increment the index counter.
        if add_to_pool {
            pool[index] = (n, n * n);
            index += 1;
        }
    }

    // Pop off the last empty/pre-allocated tuples.
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