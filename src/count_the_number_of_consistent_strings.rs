//! <https://leetcode.com/problems/count-the-number-of-consistent-strings/>
use std::collections::HashSet;
pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let allowed = allowed.chars().fold(HashSet::new(), |mut a, c| {
        a.insert(c);
        a
    });
    words
        .into_iter()
        .filter(|word| word.chars().all(|ch| allowed.contains(&ch)))
        .count() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            count_consistent_strings(
                String::from("ab"),
                vec![
                    String::from("ad"),
                    String::from("bd"),
                    String::from("aaab"),
                    String::from("baa"),
                    String::from("badab"),
                ],
            ),
            2,
        );
    }

    #[test]
    fn all_consistent() {
        assert_eq!(
            count_consistent_strings(
                String::from("abc"),
                vec![
                    String::from("a"),
                    String::from("b"),
                    String::from("c"),
                    String::from("ab"),
                    String::from("ac"),
                    String::from("bc"),
                    String::from("abc"),
                ],
            ),
            7,
        );
    }

    #[test]
    fn same_letters() {
        assert_eq!(
            count_consistent_strings(
                String::from("cad"),
                vec![
                    String::from("cc"),
                    String::from("acd"),
                    String::from("b"),
                    String::from("ba"),
                    String::from("bac"),
                    String::from("bad"),
                    String::from("ac"),
                    String::from("d"),
                ],
            ),
            4,
        );
    }
}
