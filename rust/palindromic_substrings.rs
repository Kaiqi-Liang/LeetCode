//! <https://leetcode.com/problems/palindromic-substrings/>
fn count_palindromes(s: &str, l: usize, r: usize) -> i32 {
    let mut count: i32 = 0;
    let mut l: i32 = l as _;
    let mut r = r;
    while l >= 0 && r < s.len() {
        if s.chars().nth(l as _).expect("l < s.len()") != s.chars().nth(r).expect("r < s.len()") {
            break;
        }
        count += 1;
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
