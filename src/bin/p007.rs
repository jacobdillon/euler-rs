// Problem 7 - 10001st prime
//
// What is the 10001st prime number?

extern crate euler;

use euler::is_prime;

fn main() {
    let result = (1..).filter(|&x| is_prime(x)).take(10001).last().unwrap();

    println!("{}", result);
}