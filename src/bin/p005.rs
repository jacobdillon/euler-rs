// Problem 5 - Smallest multiple
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    let result: u64 = (1..)
        .filter(|&x| !(1..21).any(|y| x % y != 0))
        .nth(0).unwrap();

    println!("{}", result);
}