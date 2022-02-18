// https://projecteuler.net/problem=9
const MAX: usize = 1000;

#[derive(Debug)]
struct Triple(usize, usize, usize);

impl Triple {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Triple(a, b, c)
    }
}

fn main() {
    for a in 1..=MAX {
        for b in 1..=MAX {
            for c in 1..=MAX {
                let t = Triple::new(a, b, c);
                let conditions = is_pythagorean_triple(&t) && (a < b && b < c) && a + b + c == 1000;
                if conditions {
                    println!("{t:?}");
                }
            }
        }
    }
}

fn is_pythagorean_triple(t: &Triple) -> bool {
    let a = t.0;
    let b = t.1;
    let c = t.2;

    usize::pow(a, 2) + usize::pow(b, 2) == usize::pow(c, 2)
}
