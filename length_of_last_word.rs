//! <https://leetcode.com/problems/length-of-last-word/>
pub fn length_of_last_word(s: String) -> usize {
    s.trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .len()
}

fn main() {
    assert_eq!(length_of_last_word(String::from(" Hello World ")), 5);
}
