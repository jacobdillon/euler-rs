// Problem 10 - Summation of primes
//
// Find the sum of all the primes below two million.

extern crate euler;

use euler::is_prime;

// WARNING: This takes HOURS to run, so I gave up and wrote p010_parallel.rs which runs in about
// 30 minutes on my 2.3Ghz i5-7360U
fn main() {
    let result: u64 = (1..2_000_000)
        .filter(|&x| is_prime(x))
        .sum();

    println!("{}", result);
}