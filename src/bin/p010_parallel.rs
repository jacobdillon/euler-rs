extern crate euler;
extern crate rayon;

use euler::is_prime;
use rayon::prelude::*;

// Problem 10 - Summation of primes
//
// Find the sum of all the primes below two million.
fn main() {
    // Parallel version of p010.rs, only takes ~30 minutes to run on a 2.3GHz i5-7360U compared to hours
    // for the non-parallel version
    let result: u64 = (1u64..2_000_000u64).into_par_iter()
        .filter(|&x| is_prime(x))
        .sum();

    println!("{}", result);
}