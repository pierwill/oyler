// https://projecteuler.net/problem=4
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

const MAX: usize = 999;

fn main() {
    let mut palindromes = vec![];

    for a in 1..=MAX {
        for b in 1..=MAX {
            let p = a * b;
            if palindrome(p) {
                palindromes.push(p);
            }
        }
    }

    println!(
        "Largest palindrome of products of 3-digit numbers: {}",
        palindromes.iter().max().unwrap()
    );
}

// Returns true if x is a palindrome.
fn palindrome(x: usize) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}
