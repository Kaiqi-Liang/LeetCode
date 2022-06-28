//! <https://leetcode.com/problems/valid-palindrome/>
pub fn is_palindrome(s: String) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(char::to_lowercase)
        .collect::<Vec<_>>();
    if s.is_empty() {
        return true;
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i].to_string() != s[j].to_string() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn main() {
    assert!(is_palindrome(String::from(
        "A man, a plan, a canal: Panama"
    )));
    assert!(!is_palindrome(String::from("race a car")));
    assert!(is_palindrome(String::from("a")));
    assert!(is_palindrome(String::from(" ")));
}
