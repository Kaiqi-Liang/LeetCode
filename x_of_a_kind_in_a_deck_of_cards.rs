//! <https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/>
use std::collections::HashMap;
pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    let counter = deck.iter().fold(HashMap::new(), |mut a, c| {
        *a.entry(c).or_insert(0) += 1;
        a
    });
    (2..=deck.len()).any(|x| deck.len() % x == 0 && counter.values().all(|count| count % x == 0))
}

fn main() {
    assert!(has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
    assert!(!has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
    assert!(has_groups_size_x(vec![1, 1]));
    assert!(has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
}
