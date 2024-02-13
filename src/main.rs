//! <https://www.hackerrank.com/challenges/max-array-sum/submissions/code/302522742?isFullScreen=true&h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=dynamic-programming>
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    false
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
fn maxSubsetSum(arr: &[i32]) -> i32 {
    /*
    opt(i) = max(opt(j), 1 <= j <= i - 2} + i
    */
    let mut opt = arr.to_owned();
    for (i, &num) in arr.iter().skip(2).enumerate() {
        opt[i + 2] = *opt[..=i].iter().max().unwrap() + num;
    }
    opt.into_iter().max().unwrap()
}

fn main() {
    assert_eq!(maxSubsetSum(&[3, 7, 4, 6, 5]), 13);
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let res = maxSubsetSum(&arr);

    println!("{res}");
    // assert!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], String::from("ABCCED")));
    // assert!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], String::from("SEE")));
    // assert!(!exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], String::from("ABCB")));
}
