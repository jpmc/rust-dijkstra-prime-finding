## Inspiration
This repository was inspired by the video [Dijkstra's Hidden Prime Finding Algorithm ](https://www.youtube.com/watch?v=fwxjMKBMR7s) by the Youtuber [@boo1](https://linktr.ee/b001io).

## What is this repository?
It is my implementation of Djikstra's Prime Finding Algorithm in Rust, which simply finds all primes up to a specified value.

This repository serves me as a scratch pad where I created my "naive" interpretation (`dijkstra_naive`) and then iteratively attempt to make improvements (`dijkstra_attempt_X`) to see what I can do to modify the algorithm performance.

I'm not planning to import any dependencies as of yet and am trying my hand at seeing what I can learn along the way.

## What is Djikstra's Prime Finding Algorithm?
In my own words, _describing how it behaves rather than the concrete implementation_, it is less like trial division and more akin to the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) except for the fact that it keeps a growing pool of primes and their multiples rather than pre-allocating the entire problem space to sieve against.

This algorithm has a performance tradeoff that puts it as having it's space and time requirements fall in-between the Trial Division and Sieving approaches.

Besides the video, and it's references, I myself have not found much in the way of online resources about this algorithm. I'm flying blind on my own intuition, applying my own rudimentary arsenal of optimizations I've accumulated from other primality-related algorithms.

## Other fun goodies

### #0 - Benchmarking _(not so fun, yet)_
Almost non-existent right now. But I figured I would point out my `main` function just does an overall time comparison between runs of my attempts.

It does not utilize a benchmarking system/suite as of now. It does not keep track of memory usage at the moment either.

Once I feel like I've gotten a minimum amount of split hairs with my existing system, I'll start using official benchmarking tools to give me more granular information to make improvements.

### #1 - Initial Capacity Allocation / Prime Counting
So far one of my pre-existing optimisations is the pool itself. Normally it is a List that continues to grow as it finds primes, and I utilized the [Inequalities sectiong of "Prime-counting" functions](https://en.wikipedia.org/wiki/Prime-counting_function#Inequalities) to pre-emptively allocate a Vector capacity to accommodate the maximum amount of primes that could exist below a given threshold. It creates some excess wasted space at the end, but it prevents any reallocation/copy/move of memory over the runtime of the algorithm.

See the `prime_count_upper_bound` function for the implementation and it's relevant comments for further details and references.

### #2 - ???