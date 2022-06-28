//! <https://leetcode.com/problems/palindromic-substrings/>
use std::convert::TryInto;
fn count_palindromes(s: &str, l: usize, r: usize) -> i32 {
    let mut count: i32 = 0;
    let mut l: i32 = l.try_into().unwrap();
    let mut r = r;
    while l >= 0 && r < s.len() {
        if s.chars().nth(l.try_into().unwrap()).unwrap() == s.chars().nth(r).unwrap() {
            count += 1;
        } else {
            break;
        }
        l -= 1;
        r += 1;
    }
    count
}

pub fn count_substrings(s: String) -> i32 {
    let mut count: i32 = 0;
    for i in 0..s.len() {
        count += count_palindromes(&s, i, i);
        count += count_palindromes(&s, i, i + 1);
    }
    count
}

fn main() {
    assert_eq!(count_substrings(String::from("abc")), 3);
    assert_eq!(count_substrings(String::from("aaa")), 6);
    assert_eq!(count_substrings(String::from("abba")), 6);
    assert_eq!(count_substrings(String::from("fdsklf")), 6);
}
