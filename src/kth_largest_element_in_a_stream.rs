//! <https://leetcode.com/problems/kth-largest-element-in-a-stream/>
use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    nums: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k.try_into().expect("k >= 1");
        let skip = if nums.len() >= k {
            nums.len() - k
        } else if nums.len() == k - 1 {
            0
        } else {
            panic!(
                "It is guaranteed that there will be at least k elements in the array when you search for the kth element."
            )
        };
        let nums = nums.into_iter().map(Reverse).collect::<BinaryHeap<_>>();
        Self {
            k,
            nums: nums.into_iter().skip(skip).collect(),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(Reverse(val));
        if self.nums.len() == self.k + 1 {
            self.nums.pop();
        }
        self.nums.peek().expect("self.nums == k >= 1").0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }

    #[test]
    fn initialise_with_empty_nums() {
        let mut obj = KthLargest::new(1, Vec::new());
        assert_eq!(obj.add(-3), -3);
        assert_eq!(obj.add(-2), -2);
        assert_eq!(obj.add(-4), -2);
        assert_eq!(obj.add(0), 0);
        assert_eq!(obj.add(4), 4);
    }

    #[test]
    fn initialise_with_one_num() {
        let mut obj = KthLargest::new(2, vec![0]);
        assert_eq!(obj.add(-1), -1);
        assert_eq!(obj.add(1), 0);
        assert_eq!(obj.add(-2), 0);
        assert_eq!(obj.add(-4), 0);
        assert_eq!(obj.add(3), 1);
    }
}
