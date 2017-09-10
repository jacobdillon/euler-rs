// Problem 1 - Multiples of 3 and 5
//
// Find the sum of all the multiples of 3 or 5 below 1000.
fn main() {
    let result = (1..1000).filter(|&x| x % 3 == 0 || x % 5 == 0).fold(0, |acc, x| acc + x);
    println!("{}", result);
}