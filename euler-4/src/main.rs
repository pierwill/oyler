fn main() {
    assert!(palindrome(11));
}

// Returns true if x is a palindrome.
fn palindrome(x: usize) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}
