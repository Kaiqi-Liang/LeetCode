//! <https://leetcode.com/problems/kth-distinct-string-in-an-array/>
use std::collections::HashMap;
pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let counter = counter!(arr.iter());
    arr.iter()
        .filter(|elem| counter.get(elem).is_some_and(|&count| count == 1))
        .nth((k - 1) as _)
        .unwrap_or(&String::new())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            kth_distinct(
                vec![
                    String::from("d"),
                    String::from("b"),
                    String::from("c"),
                    String::from("b"),
                    String::from("c"),
                    String::from("a"),
                ],
                2,
            ),
            String::from("a"),
        );
    }

    #[test]
    fn all_distinct() {
        assert_eq!(
            kth_distinct(
                vec![String::from("aaa"), String::from("aa"), String::from("a")],
                1,
            ),
            String::from("aaa"),
        );
    }

    #[test]
    fn fewer_distinct_strings_than_k() {
        assert_eq!(
            kth_distinct(
                vec![String::from("a"), String::from("b"), String::from("a"),],
                3,
            ),
            String::new(),
        );
    }
}
