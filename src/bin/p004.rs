// Problem 4 - Largest palindrome product
//
// Find the largest palindrome made from the product of two 3-digit numbers.
fn main() {
    let result = (100..1000)
        .map(|x| x * x)
        .filter(|&x| is_palindromic(x))
        .max().unwrap();

    println!("{}", result);
}

fn is_palindromic(n: u64) -> bool {
    let mut rev = 0;
    let mut current = n;

    while current != 0 {
        rev *= 10;
        rev += current % 10;
        current /= 10;
    }

    n == rev
}
