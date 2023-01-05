//! <https://leetcode.com/problems/valid-palindrome-ii/>
fn check_palindrome(s: &[u8], mut i: usize, mut j: usize) -> i32 {
    while i < j && s[i] == s[j] {
        i += 1;
        j -= 1;
    }
    if i >= j {
        -1
    } else {
        i as i32
    }
}

pub fn valid_palindrome(s: String) -> bool {
    let i = check_palindrome(s.as_bytes(), 0, s.len() - 1);
    if i == -1 {
        true
    } else {
        let i: usize = i as _;
        check_palindrome(s.as_bytes(), i, s.len() - i - 2) == -1
            || check_palindrome(s.as_bytes(), i + 1, s.len() - i - 1) == -1
    }
}

fn main() {
    assert!(valid_palindrome(String::from("aba")));
    assert!(valid_palindrome(String::from("abca")));
    assert!(!valid_palindrome(String::from("abc")));
}
