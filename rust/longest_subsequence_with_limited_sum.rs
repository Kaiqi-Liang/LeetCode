//! <https://leetcode.com/problems/longest-subsequence-with-limited-sum/>
fn binary_search(haystack: &[i32], needle: &i32) -> usize {
    let mut low = 0;
    let mut high = haystack.len() - 1;
    while low + 1 < high {
        let middle = (low + high) / 2;
        match haystack[middle].cmp(needle) {
            std::cmp::Ordering::Less => low = middle,
            std::cmp::Ordering::Equal => return middle + 1,
            std::cmp::Ordering::Greater => high = middle,
        }
    }
    if haystack[high] <= *needle {
        high + 1
    } else if haystack[low] <= *needle {
        low + 1
    } else {
        0
    }
}

pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    nums.sort();
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    queries
        .iter()
        .map(|query| binary_search(&nums, query) as _)
        .collect()
}

fn main() {
    assert_eq!(
        answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
        vec![2, 3, 4],
    );
    assert_eq!(answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    assert_eq!(
        answer_queries(
            vec![624082],
            vec![972985, 564269, 607119, 693641, 787608, 46517, 500857, 140097],
        ),
        vec![1, 0, 0, 1, 1, 0, 0, 0],
    );
    assert_eq!(answer_queries(vec![1000000], vec![1000000]), vec![1]);
}
