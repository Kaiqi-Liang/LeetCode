use std::collections::HashMap;

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut words = words.into_iter().map(|word| {
        word.chars().fold(HashMap::new(), |mut a, c| {
            a.entry(c).and_modify(|count| *count += 1).or_insert(1);
            a
        })
    });
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
    use std::collections::HashSet;

    use super::*;

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
