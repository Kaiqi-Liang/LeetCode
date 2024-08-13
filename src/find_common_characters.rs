//! <https://leetcode.com/problems/find-common-characters/>
use std::collections::HashMap;
pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut words = words.into_iter().map(|word| counter!(word.chars()));
    if let Some(mut first) = words.next() {
        for next in words {
            for (key, value) in first.iter_mut() {
                *value = if let Some(count) = next.get(key) {
                    *count.min(value)
                } else {
                    0
                };
            }
        }
        first.into_iter().fold(Vec::new(), |mut a, (ch, count)| {
            for _ in 0..count {
                a.push(ch.to_string())
            }
            a
        })
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn duplicates() {
        assert_eq!(
            common_chars(vec![
                String::from("bella"),
                String::from("label"),
                String::from("roller"),
            ])
            .into_iter()
            .collect::<HashSet<_>>(),
            HashSet::from([String::from("l"), String::from("l"), String::from("e")])
        );
    }

    #[test]
    fn happy_path() {
        assert_eq!(
            common_chars(vec![
                String::from("cool"),
                String::from("lock"),
                String::from("cook"),
            ])
            .into_iter()
            .collect::<HashSet<_>>(),
            HashSet::from([String::from("c"), String::from("o")])
        );
    }
}
