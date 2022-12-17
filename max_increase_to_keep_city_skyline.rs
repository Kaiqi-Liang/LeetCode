//! <https://leetcode.com/problems/max-increase-to-keep-city-skyline/>
pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut max_rows = Vec::with_capacity(n);
    let mut max_cols = Vec::with_capacity(n);
    for i in 0..n {
        max_rows.push(*grid[i].iter().max().unwrap());
        let mut max_col = 0;
        for j in 0..n {
            max_col = max_col.max(grid[j][i]);
        }
        max_cols.push(max_col);
    }

    let mut total = 0;
    for i in 0..n {
        for j in 0..n {
            total += max_rows[i].min(max_cols[j]) - grid[i][j];
        }
    }
    total
}

fn main() {
    assert_eq!(
        max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0],
        ]),
        35,
    );
    assert_eq!(
        max_increase_keeping_skyline(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        0,
    );
}
