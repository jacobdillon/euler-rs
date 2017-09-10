pub fn is_prime(x: u64) -> bool{
    if x > 2 && x % 2 == 0 {
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