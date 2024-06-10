//! <https://leetcode.com/problems/happy-number/>
use std::collections::HashSet;
pub fn is_happy(mut n: i32) -> bool {
    let mut set = HashSet::new();
    loop {
        set.insert(n);
        n = n
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).expect("ch is converted from an i32"))
            .fold(0, |acc, digit| acc + digit.pow(2) as i32);
        if n == 1 {
            break true;
        } else if set.contains(&n) {
            break false;
        }
    }
}

fn main() {
    assert!(is_happy(19));
    assert!(!is_happy(2));
}
