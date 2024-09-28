//! <https://leetcode.com/problems/uncommon-words-from-two-sentences/>
use std::collections::HashMap;
pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    counter!(s1.split_whitespace().chain(s2.split_whitespace()))
        .into_iter()
        .filter_map(|(word, appearances)| {
            if appearances == 1 {
                Some(word.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn common_words_from_both_sentences() {
        assert_eq!(
            uncommon_from_sentences(
                String::from("this apple is sweet"),
                String::from("this apple is sour"),
            )
            .into_iter()
            .collect::<HashSet<_>>(),
            HashSet::from([String::from("sweet"), String::from("sour")]),
        );
    }

    #[test]
    fn common_words_from_first_sentence() {
        assert_eq!(
            uncommon_from_sentences(String::from("apple apple"), String::from("banana")),
            vec![String::from("banana")],
        );
    }
}
