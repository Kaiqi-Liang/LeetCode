//! <https://leetcode.com/problems/relative-ranks/>
use std::collections::{BinaryHeap, HashMap};

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut map = HashMap::new();
    let mut sorted_score = BinaryHeap::new();
    for (i, score) in score.iter().enumerate() {
        map.insert(score, i);
        sorted_score.push(score);
    }
    let mut answer = vec![String::new(); score.len()];
    for rank in 1..=score.len() {
        answer[map[sorted_score.pop().expect("sorted_score.len() > 0")]] = match rank {
            1 => String::from("Gold Medal"),
            2 => String::from("Silver Medal"),
            3 => String::from("Bronze Medal"),
            _ => rank.to_string(),
        };
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decreasing_order() {
        assert_eq!(
            find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
    }

    #[test]
    fn random_order() {
        assert_eq!(
            find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }
}
