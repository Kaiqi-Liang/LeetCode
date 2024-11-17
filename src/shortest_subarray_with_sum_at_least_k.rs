//! <https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/>
use std::collections::VecDeque;
pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let mut stack = VecDeque::new();
    let mut curr_sum: i64 = 0;
    let mut res = i32::MAX;
    for (curr_index, num) in nums.into_iter().enumerate() {
        curr_sum += num as i64;
        if curr_sum >= k {
            res = res.min((curr_index + 1) as i32);
        }
        while let Some(&(prev_index, prev_sum)) = stack.front() {
            if curr_sum - prev_sum >= k {
                res = res.min((curr_index - prev_index) as i32);
                stack.pop_front();
            } else {
                break;
            }
        }
        while let Some(&(_, prev_sum)) = stack.back() {
            if curr_sum < prev_sum {
                stack.pop_back();
            } else {
                break;
            }
        }
        stack.push_back((curr_index, curr_sum));
    }
    if res == i32::MAX {
        -1
    } else {
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_num() {
        assert_eq!(shortest_subarray(vec![1], 1), 1);
    }

    #[test]
    fn no_subarray() {
        assert_eq!(shortest_subarray(vec![1, 2], 4), -1);
    }

    #[test]
    fn negative_num() {
        assert_eq!(shortest_subarray(vec![2, -1, 2], 3), 3);
    }

    #[test]
    fn happy_path() {
        assert_eq!(shortest_subarray(vec![-1, 2, -1, 2, 3], 3), 1);
    }
}
