#[allow(dead_code)]
const UP_TO_100: &str = "2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97";
#[allow(dead_code)]
const FIRST_100: &str = "2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541";

// Inspiration: https://youtu.be/fwxjMKBMR7s
fn main() {

    const SAMPLE: u64 = 100_000;
    let now = std::time::Instant::now();
    let primes = dijkstra_naive(SAMPLE);
    let elapsed = now.elapsed();
    println!("DEBUG: Elapsed: {:.2?}", elapsed);
    //print_vec(primes);
    println!("Primes Found: {}\n", primes.len());

    let now = std::time::Instant::now();
    let primes = dijkstra_attempt_1(SAMPLE);
    let elapsed = now.elapsed();
    println!("DEBUG: Elapsed: {:.2?}", elapsed);
    //print_vec(primes);
    println!("Primes Found: {}", primes.len());

    //println!("\nHard-coded Comparison Results:");
    //println!("{}", FIRST_100);

}

// Attempt 1: Add global sqrt boundary to speed up iterations.
fn dijkstra_attempt_1(upto: u64) -> Vec<(u64, u64)> {
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

    println!("DEBUG: Removed {} dead tuples.", pool.len() - index);
    for _ in index..pool.len() {
        pool.pop();
    }
    println!("DEBUG: {} primes found.", pool.len());
    pool
}


// First working implementation (or Attempt 0)
#[allow(dead_code)]
fn dijkstra_naive(upto: u64) -> Vec<(u64, u64)> {
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
    println!("DEBUG: Removed {} dead tuples.", pool.len() - index);
    for _ in index..pool.len() {
        pool.pop();
    }
    println!("DEBUG: {} primes found.", pool.len());
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

// A pretty-printer for Vec<(u32, u32)> results.
// Specifically meant for just printing the first u32 in the tuple, as a comma-delimited string.
#[allow(dead_code)]
fn print_vec(v: Vec<(u64, u64)>) {
    println!("{}", v.iter()
        .map(|t| t.0.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
}

// Generic version from ChatGPT that simply disregards the tuple structure
// and just says "first value must be printable".
// It perfectly replicates my manual attempt but in a more generic manner.
#[allow(dead_code)]
fn print_vec_gpt<T, U>(v: Vec<(T, U)>)
    where T: std::fmt::Display,
{
    let result = v
        .iter()
        .map(|(first, _)| first.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", result);
}