extern crate euler;

use euler::is_prime;

// Problem 7 - 10001st prime
//
// What is the 10001st prime number?
fn main() {
    let result = (1..).filter(|&x| is_prime(x)).take(10001).last().unwrap();

    println!("{}", result);
}