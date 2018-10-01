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
    let rev_n: String = n.to_string().chars().rev().collect();
    n.to_string() == rev_n
}
