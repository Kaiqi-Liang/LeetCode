//! <https://leetcode.com/problems/perfect-squares/>
use std::collections::HashSet;
pub fn num_squares(n: i32) -> i32 {
    let n: usize = n as _;
    let mut denominations = HashSet::new();
    let mut i: usize = 1;
    let mut square = i.pow(2);
    while square <= n {
        denominations.insert(square);
        i += 1;
        square = i.pow(2);
    }
    making_change(denominations, n) as _
}

fn making_change(denominations: HashSet<usize>, amount: usize) -> usize {
    let mut opt = vec![vec![0; amount + 1]; denominations.len() + 1];
    opt[0] = vec![usize::MAX; amount + 1];
    for (mut i, denomination) in denominations.iter().enumerate() {
        i += 1;
        for j in 1..=amount {
            let curr = if j < *denomination {
                usize::MAX
            } else {
                opt[i][j - denomination]
            };
            opt[i][j] = std::cmp::min(
                opt[i - 1][j],
                if curr == usize::MAX {
                    usize::MAX
                } else {
                    curr + 1
                },
            );
        }
    }
    opt[denominations.len()][amount]
}

fn main() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
