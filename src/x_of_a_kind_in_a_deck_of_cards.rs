//! <https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/>
use std::collections::HashMap;
pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    (2..=deck.len())
        .any(|x| deck.len() % x == 0 && counter!(deck.iter()).values().all(|count| count % x == 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert!(has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
    }

    #[test]
    fn impossible_partition() {
        assert!(!has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
    }

    #[test]
    fn repeated_cards() {
        assert!(has_groups_size_x(vec![1, 1]));
    }

    #[test]
    fn multiple_repeated_cards() {
        assert!(has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
    }
}
