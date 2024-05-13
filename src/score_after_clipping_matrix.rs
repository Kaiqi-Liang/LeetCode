//! https://leetcode.com/problems/score-after-flipping-matrix/>
pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    for row in grid.iter_mut() {
        let first_column = row.first().expect("grid[i].len() > 0");
        if *first_column == 0 {
            for column in row {
                *column = if *column == 0 { 1 } else { 0 };
            }
        }
    }
    for column in 0..grid.first().expect("grid.len() > 0").len() {
        let mut count_zeros = 0;
        for row in grid.iter() {
            if row[column] == 0 {
                count_zeros += 1;
            }
        }
        if count_zeros > grid.len() - count_zeros {
            for row in grid.iter_mut() {
                row[column] = if row[column] == 0 { 1 } else { 0 };
            }
        }
    }
    let mut score = 0;
    for row in grid {
        for column in 0..row.len() {
            score += 2u32.pow((row.len() - column - 1) as _) as i32 * row[column];
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39,
        );
    }

    #[test]
    fn remove_first() {
        assert_eq!(matrix_score(vec![vec![0]]), 1);
    }
}
