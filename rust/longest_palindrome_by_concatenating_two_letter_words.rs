//! <https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/>
use std::collections::HashMap;
macro_rules! get_letter {
    ($word:ident, $index:literal) => {
        $word
            .chars()
            .nth($index)
            .expect("Each element of words consists of two lowercase English letters.")
    };
}

macro_rules! get_letters {
    ($word:ident) => {
        (get_letter!($word, 0), get_letter!($word, 1))
    };
}

pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut non_palindrome_words: HashMap<&String, usize> = HashMap::new();
    for word in &words {
        let (letter1, letter2) = get_letters!(word);
        if letter1 != letter2 {
            non_palindrome_words
                .entry(word)
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        }
    }

    let palindrome_words = words
        .iter()
        .filter_map(|word| {
            let (letter1, letter2) = get_letters!(word);
            if letter1 == letter2 {
                Some(letter1)
            } else {
                None
            }
        })
        .fold(HashMap::<char, usize>::new(), |mut a, c| {
            a.entry(c).and_modify(|count| *count += 1).or_insert(1);
            a
        });

    let mut central = false;
    ((non_palindrome_words.iter().fold(0, |a, (&word, &count)| {
        let (letter1, letter2) = get_letters!(word);
        a + count.min(
            *non_palindrome_words
                .get(&format!("{letter2}{letter1}"))
                .unwrap_or(&0),
        )
    }) + palindrome_words.values().into_iter().fold(0, |a, &c| {
        a + if c % 2 == 0 {
            c
        } else {
            central = true;
            c - 1
        }
    }) + if central { 1 } else { 0 })
        * 2) as _
}

fn main() {
    assert_eq!(
        longest_palindrome(vec![
            String::from("lc"),
            String::from("cl"),
            String::from("gg"),
        ]),
        6,
    );
    assert_eq!(
        longest_palindrome(vec![
            String::from("ab"),
            String::from("ty"),
            String::from("yt"),
            String::from("lc"),
            String::from("cl"),
            String::from("ab"),
        ]),
        8,
    );
    assert_eq!(
        longest_palindrome(vec![
            String::from("cc"),
            String::from("ll"),
            String::from("xx"),
        ]),
        2,
    );
    assert_eq!(
        longest_palindrome(vec![
            String::from("ab"),
            String::from("ty"),
            String::from("ty"),
            String::from("yt"),
            String::from("lc"),
            String::from("cl"),
            String::from("ab"),
            String::from("cc"),
            String::from("cc"),
            String::from("cc"),
            String::from("dd"),
            String::from("dd"),
            String::from("dd"),
        ]),
        18,
    );
}
