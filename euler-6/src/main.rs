// https://projecteuler.net/problem=6
fn main() {
    println!("{}", diff(100));
}

// Difference between the sum of the squares and the square of the sum
// of the first n natural numbers.
fn diff(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}

// Return sum of squares of first n natural numbers.
fn sum_of_squares(n: usize) -> usize {
    let mut sum: usize = 0;
    for x in 1..(n + 1) {
        sum += usize::pow(x, 2);
    }
    sum
}

// Return square of sum of first n natural numbers.
fn square_of_sum(n: usize) -> usize {
    let mut sum: usize = 0;
    for x in 1..(n + 1) {
        sum += x;
    }
    usize::pow(sum, 2)
}
