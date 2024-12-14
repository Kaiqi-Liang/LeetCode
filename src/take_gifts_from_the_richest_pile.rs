//! <https://leetcode.com/problems/take-gifts-from-the-richest-pile/>
use std::collections::BinaryHeap;
pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut priority_queue = BinaryHeap::from(gifts);
    for _ in 0..k {
        if let Some(pile) = priority_queue.pop() {
            let leave_behind = (pile as f64).sqrt().floor();
            priority_queue.push(leave_behind as i32);
        } else {
            break;
        }
    }
    priority_queue.into_iter().map(|gifts| gifts as i64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
    }

    #[test]
    fn cannot_take_gifts() {
        assert_eq!(pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }
}
