// https://projecteuler.net/problem=10

//! I used a crate; so sue me.

use primal::Primes;

const MAX: usize = 2 * usize::pow(10, 6);

fn main() {
    let primes: Vec<usize> = Primes::all().take_while(|p| *p < MAX).into_iter().collect();

    println!("{}", primes.iter().sum::<usize>());
}
