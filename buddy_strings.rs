//! https://leetcode.com/problems/patching-array/
use std::collections::HashMap;
fn find_duplicates(s: &str) -> Vec<char> {
    let mut map = HashMap::new();
    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }
    map.iter()
        .filter(|&(_k, v)| *v > 1)
        .map(|(k, _v)| *k)
        .collect()
}

pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    let mut diff1: (u8, u8) = (0, 0);
    let mut diff2: (u8, u8) = (0, 0);
    for (char1, char2) in s.bytes().zip(goal.bytes()) {
        if char1 != char2 {
            if diff1 == (0, 0) {
                diff1 = (char1, char2);
            } else if diff2 == (0, 0) {
                diff2 = (char1, char2);
            } else {
                return false;
            }
        }
    }
    if diff1 == (0, 0) {
        find_duplicates(&s)
            .iter()
            .any(|ch| find_duplicates(&goal).contains(ch))
    } else {
        diff1.0 == diff2.1 && diff1.1 == diff2.0
    }
}

fn main() {
    assert!(buddy_strings(String::from("abcd"), String::from("cbad")));
    assert!(buddy_strings(String::from("ab"), String::from("ba")));
    assert!(!buddy_strings(String::from("ab"), String::from("ab")));
    assert!(buddy_strings(String::from("aa"), String::from("aa")));
    assert!(!buddy_strings(String::from("aa"), String::from("aaa")));
}
