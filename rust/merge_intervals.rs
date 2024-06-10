//! <https://leetcode.com/problems/merge-intervals/>
pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut i = 0;
    while i < intervals.len() - 1 {
        let (_, curr_end) = (intervals[i][0], intervals[i][1]);
        let (next_start, next_end) = (intervals[i + 1][0], intervals[i + 1][1]);
        if curr_end >= next_start {
            intervals.remove(i + 1);
            *intervals[i]
                .get_mut(1)
                .expect("Each element of intervals consists of a start and an end.") =
                next_end.max(curr_end);
        } else {
            i += 1;
        }
    }
    intervals
}

fn main() {
    assert_eq!(
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]],
    );
    assert_eq!(
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![10, 18]]),
        vec![vec![1, 6], vec![8, 18]],
    );
    assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
    assert_eq!(merge(vec![vec![1, 4], vec![0, 4]]), vec![vec![0, 4]]);
    assert_eq!(merge(vec![vec![1, 4], vec![0, 4]]), vec![vec![0, 4]]);
    assert_eq!(merge(vec![vec![1, 4], vec![0, 1]]), vec![vec![0, 4]]);
    assert_eq!(
        merge(vec![
            vec![2, 3],
            vec![4, 5],
            vec![6, 7],
            vec![8, 9],
            vec![1, 10],
        ]),
        vec![vec![1, 10]],
    );
}
