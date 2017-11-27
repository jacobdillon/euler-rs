extern crate num;

use num::Integer;

pub fn is_prime(x: u64) -> bool{
    if x > 2 && x.is_even() {
        return false
    }

    if x == 1 {
        return false
    }

    for n in 2..x/2+1 {
        if x % n == 0 {
            return false
        }
    }

    true
}