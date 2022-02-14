const MAX: usize = 1000;

fn main() {
    let mut set: Vec<usize> = vec![];

    for n in 0..MAX {
        let conditions = n != 0 && (n % 3 == 0 || n % 5 == 0);
        if conditions {
            set.push(n);
        }
    }

    println!("{}", set.iter().sum::<usize>());
}
