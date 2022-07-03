//! <https://leetcode.com/problems/number-of-different-integers-in-a-string/>
use std::collections::HashSet;
pub fn num_different_integers(word: String) -> i32 {
    word.chars()
        .map(|ch| match ch {
            'a'..='z' => ' ',
            _ => ch,
        })
        .collect::<String>()
        .split_ascii_whitespace()
        .map(|digit| digit.trim_start_matches('0'))
        .collect::<HashSet<&str>>()
        .len() as i32
}

fn main() {
    assert_eq!(num_different_integers(String::from("a123bc34d8ef34")), 3);
    assert_eq!(num_different_integers(String::from("leet1234code234")), 2);
    assert_eq!(num_different_integers(String::from("a1b01c001")), 1);
}
