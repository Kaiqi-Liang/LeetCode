//! <https://leetcode.com/problems/palindrome-number/>
pub fn is_palindromic(x: i32) -> bool {
    let x = x.to_string();
    let mut left = 0;
    let mut right = x.len() - 1;
    while left < right {
        if x.chars().nth(left) != x.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    assert!(is_palindromic(121));
    assert!(is_palindromic(0));
    assert!(!is_palindromic(-121));
    assert!(!is_palindromic(10));
}
