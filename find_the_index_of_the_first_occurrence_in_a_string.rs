//! <https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/>
pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as _,
        None => -1,
    }
}

fn main() {
    assert_eq!(str_str(String::from("sadbutsad"), String::from("sad")), 0);
    assert_eq!(str_str(String::from("leetcode"), String::from("leeto")), -1);
    assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
    assert_eq!(str_str(String::from("aaaaa"), String::from("bba")), -1);
    assert_eq!(str_str(String::from(""), String::from("a")), -1);
}
