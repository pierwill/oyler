//! https://projecteuler.net/problem=2

use common::fibonacci;

// The first n Fibonacci numbers.
const MAX: usize = 4 * usize::pow(10, 6);

fn main() {
    // Find greatest n such that fibonacci(n) < 4 * 10^6
    let mut greatest_n: usize = 0;
    while fibonacci(greatest_n) < MAX {
        greatest_n += 1;
    }

    let mut even_fibs = vec![];
    for n in 0..(greatest_n) {
        let f = fibonacci(n);
        if f % 2 == 0 {
            even_fibs.push(f);
        }
    }

    println!(
        "The even Fibonacci numbers less than 4 million are:\n {:?}",
        even_fibs
    );
    println!("Their sum is {}.", even_fibs.iter().sum::<usize>());
}
