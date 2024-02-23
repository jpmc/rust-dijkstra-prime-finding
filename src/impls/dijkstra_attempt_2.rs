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

// Attempt 2: Add 3rd tuple value (prime, prime^2, multiple), "skip" evens.
// * We determined that instead of the "global sqrt" from Attempt 1, that individualized sqrt(N)
//   saves iterations+time.
// * However, to save from individualized (value > sqrt(N)) checks, we rewrite it to (value^2 > N).
// * Furthermore, to save on multiplications-in-iteration, we simply use a 3rd u64 value stored in
//   the tuple that is fixed at prime^2, as a permanent sqrt(N) check avoidance measure.
// - We now skip every even iteration in full, in exchange for a single "if" check on (n+1) to keep
//   the multiples bookkeeping intact.
pub fn run(upto: u64) -> Vec<(u64, u64, u64)> {
    // !! Removed "global sqrt".
    let size: usize = prime_count_upper_bound(upto) as usize;
    let mut pool: Vec<(u64, u64, u64)> = Vec::with_capacity(size).clone();
    pool.resize(size, (0, 0, 0));
    // !! Pre-populate the "multiple" for 2 up to the maximum multiple value in an effort to "skip" it.
    pool[0] = (2, 4, upto.next_multiple_of(2));

    let mut index = 1;
    // !! Introduced ".step_by(2)" to skip iterations on even numbers.
    for n in (3..=upto).step_by(2) {
        let mut add_to_pool: bool = true;
        for tuple in &mut pool {
            // !! Using new "prime^2" tuple member to do a personalized (value > sqrt(N)) check.
            if tuple.0 == 0 || tuple.1 > n {
                break;
            }
            // Pool sweep to update multiples if a composite is found.
            if n >= tuple.2 {
                add_to_pool = false;
                tuple.2 += tuple.0;
            }
            // !! In order to "skip" multiples of 2, we need to take into account if the even entry
            //    would have tripped incrementing the multiple index for a tuple.
            if (n + 1) >= tuple.2 {
                tuple.2 = (n+2).next_multiple_of(tuple.0);
            }
        }

        if add_to_pool {
            pool[index] = (n, n * n, n * n);
            index += 1;
        }
    }

    // !! Learned Vec::truncate() exists, instead of popping the excess entries manually.
    pool.truncate(index);

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