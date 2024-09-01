//! <https://leetcode.com/problems/convert-1d-array-into-2d-array/>

pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    let m: usize = m as _;
    let n: usize = n as _;
    if m * n != original.len() {
        vec![]
    } else {
        original
            .into_iter()
            .enumerate()
            .fold(vec![Vec::with_capacity(n); m], |mut a, c| {
                a[c.0 / n].push(c.1);
                a
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            vec![vec![1, 2], vec![3, 4]],
        );
    }

    #[test]
    fn same_as_original() {
        assert_eq!(construct2_d_array(vec![1, 2, 3], 1, 3), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn impossible() {
        assert_eq!(construct2_d_array(vec![1, 2], 1, 1), Vec::<Vec<i32>>::new());
    }
}
