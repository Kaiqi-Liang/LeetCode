//! <https://leetcode.com/problems/coin-change/>
use std::cmp;
use std::convert::TryInto;
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount: usize = amount.try_into().unwrap();
    let coins: Vec<usize> = coins
        .iter()
        .map(|&coin| TryInto::<usize>::try_into(coin).unwrap())
        .collect();
    let mut opt = vec![vec![0; amount + 1]; coins.len() + 1];
    opt[0] = vec![usize::MAX; amount + 1];
    for (mut i, coin) in coins.iter().enumerate() {
        i += 1;
        for j in 1..=amount {
            let curr = if j < *coin {
                usize::MAX
            } else {
                opt[i][j - coin]
            };
            opt[i][j] = cmp::min(
                opt[i - 1][j],
                if curr == usize::MAX {
                    usize::MAX
                } else {
                    curr + 1
                },
            );
        }
    }
    opt[coins.len()][amount] as i32
}

fn main() {
    assert_eq!(coin_change(vec![1, 5, 12, 19], 16), 4);
    assert_eq!(coin_change(vec![2, 3], 4), 2);
    assert_eq!(coin_change(vec![3, 5, 12], 11), 3);
    assert_eq!(coin_change(vec![2, 5], 6), 3);
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(coin_change(vec![2], 3), -1);
    assert_eq!(coin_change(vec![1], 0), 0);
    assert_eq!(coin_change(vec![186, 419, 83, 408], 6249), 20);
}
