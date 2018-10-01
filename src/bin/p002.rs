// Problem 2 - Even Fibonacci numbers
//
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find
// the sum of the even-valued terms.

extern crate num;

use num::Integer;

struct Fib {
    current: u32,
    next: u32,
}

impl Iterator for Fib {
    type Item = Fib;

    fn next(&mut self) -> Option<Self::Item> {
        let next_fib = self.current + self.next;

        self.current = self.next;
        self.next = next_fib;

        Some(Fib { current: self.current, next: self.next })
    }
}

fn main() {
    let result = Fib { current: 1, next: 1 }
        .take_while(|f| f.current < 4_000_000)
        .filter(|f| f.current.is_even())
        .fold(0, |acc, f| acc + f.current);

    println!("{}", result);
}