extern crate num;

use num::Integer;
use num::bigint::BigInt;
use num::bigint::Sign;

// Problem 16 - Power digit sum
//
// What is the sum of the digits of the number 21000?
fn main() {
    let exp_result = pow_by_squaring(BigInt::new(Sign::Plus, vec![2]), BigInt::new(Sign::Plus, vec![1000]));

    let result = exp_result.to_string().chars().fold(
        BigInt::new(Sign::NoSign, vec![0]),
        |acc, c| acc + BigInt::new(Sign::Plus, vec![c.to_digit(10).unwrap()])
    );

    println!("{}", result);
}

fn pow_by_squaring(x: BigInt, n: BigInt) -> BigInt {
    if n < BigInt::new(Sign::NoSign, vec![0]) {
        return pow_by_squaring(1 / x, -n)
    } else if n == BigInt::new(Sign::NoSign, vec![0]) {
        return BigInt::new(Sign::Plus,vec![1])
    } else if n == BigInt::new(Sign::Plus, vec![1]) {
        return x
    } else if n.is_even() {
        return pow_by_squaring(&x * &x,  n / 2)
    } else {
        return &x * pow_by_squaring(&x * &x, (n - 1) / 2)
    }
}