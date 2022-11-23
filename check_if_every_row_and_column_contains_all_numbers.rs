//! <https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/>
use std::collections::HashSet;
pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    for i in 0..matrix.len() {
        let mut hashset = HashSet::new();
        for j in 0..matrix[i].len() {
            if hashset.contains(&matrix[j][i]) {
                return false;
            }
            hashset.insert(matrix[j][i]);
        }
    }
    matrix
        .iter()
        .all(|row| row.iter().collect::<HashSet<&i32>>().len() == row.len())
}

fn main() {
    assert!(check_valid(vec![
        vec![1, 2, 3],
        vec![3, 1, 2],
        vec![2, 3, 1]
    ]));
    assert!(!check_valid(vec![
        vec![1, 1, 1],
        vec![1, 2, 3],
        vec![1, 2, 3]
    ]));
}
