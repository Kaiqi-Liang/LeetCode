//! <https://leetcode.com/problems/remove-letter-to-equalize-frequency/>
use std::collections::HashMap;
pub fn equal_frequency(word: String) -> bool {
    let mut frequency = HashMap::new();
    for ch in word.chars() {
        frequency
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut map = HashMap::new();
    for ch in frequency.values() {
        map.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }
    match map.keys().len() {
        2 => {
            let first = *map.keys().next().unwrap();
            let second = *map.keys().nth(1).unwrap();
            map.iter().any(|(&&key, &value)| key == 1 && value == 1)
                || ((*first - *second) as i32).abs() == 1
                    && map.get(std::cmp::max(first, second)).unwrap() == &1
        }
        1 => map.keys().next().unwrap() == &&1 || map.values().next().unwrap() == &1,
        _ => false,
    }
}

fn main() {
    assert!(!equal_frequency(String::from("aazz")));
    assert!(equal_frequency(String::from("zz")));
    assert!(equal_frequency(String::from("abcc")));
    assert!(equal_frequency(String::from("abc")));
    assert!(equal_frequency(String::from("aba")));
    assert!(equal_frequency(String::from("abaab")));
    assert!(equal_frequency(String::from("aaabbbcccd")));
    assert!(!equal_frequency(String::from("aaabbccc")));
    assert!(equal_frequency(String::from("aaabccc")));
    assert!(equal_frequency(String::from("aabbccdde")));
    assert!(!equal_frequency(String::from("cbccca")));
    assert!(!equal_frequency(String::from("abbccc")));
}
