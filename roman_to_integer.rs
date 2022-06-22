//! https://leetcode.com/problems/roman-to-integer/
use std::collections::HashMap;
pub fn roman_to_int(s: String) -> i32 {
    let roman = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut integer = 0;
    let mut i = 0;
    while i < s.len() {
        let ch = s.chars().nth(i).unwrap();
        if let Some(next) = s.chars().nth(i + 1) {
            if ch == 'I' && (next == 'V' || next == 'X')
                || ch == 'X' && (next == 'L' || next == 'C')
                || ch == 'C' && (next == 'D' || next == 'M')
            {
                integer += *roman.get(&next).unwrap() - *roman.get(&ch).unwrap();
                i += 2;
                continue;
            }
        }
        integer += *roman.get(&ch).unwrap();
        i += 1;
    }
    integer
}

fn main() {
    assert_eq!(roman_to_int(String::from("II")), 2);
    assert_eq!(roman_to_int(String::from("VI")), 6);
    assert_eq!(roman_to_int(String::from("XXVII")), 27);
    assert_eq!(roman_to_int(String::from("IV")), 4);
}
