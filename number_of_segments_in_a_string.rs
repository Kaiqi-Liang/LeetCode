//! <https://leetcode.com/problems/number-of-segments-in-a-string/>
pub fn count_segments(s: String) -> i32 {
    s.split_ascii_whitespace().count() as i32
}

fn main() {
    assert_eq!(count_segments(String::from("Hello, my name is John")), 5);
    assert_eq!(count_segments(String::from("Hello")), 1);
    assert_eq!(count_segments(String::from("")), 0);
}
