//! <https://leetcode.com/problems/word-pattern/>
use std::collections::HashMap;
pub fn word_pattern(pattern: String, s: String) -> bool {
    let letters: Vec<_> = pattern.chars().map(|word| word.to_string()).collect();
    let iter = Box::new(letters.into_iter());
    let pattern = generate_pattern(iter.clone(), generate_dictionary(iter));

    let words: Vec<_> = s.split(' ').map(|word| word.to_string()).collect();
    let iter = Box::new(words.into_iter());
    let s = generate_pattern(iter.clone(), generate_dictionary(iter));
    pattern == s
}

fn generate_pattern(
    s: Box<dyn Iterator<Item = String>>,
    dictionary: HashMap<String, usize>,
) -> Vec<usize> {
    s.map(|word| *dictionary.get(&word).unwrap()).collect()
}

fn generate_dictionary(s: Box<dyn Iterator<Item = String>>) -> HashMap<String, usize> {
    let mut dictionary = HashMap::new();
    for word in s {
        let length = dictionary.len();
        dictionary.entry(word).or_insert(length);
    }
    dictionary
}

fn main() {
    assert!(word_pattern(
        String::from("abba"),
        String::from("dog cat cat dog")
    ));
    assert!(!word_pattern(
        String::from("abba"),
        String::from("dog cat cat fish")
    ));
    assert!(!word_pattern(
        String::from("aaaa"),
        String::from("dog cat cat fish")
    ));
    assert!(word_pattern(
        String::from("abc"),
        String::from("dog cat fish")
    ));
}
