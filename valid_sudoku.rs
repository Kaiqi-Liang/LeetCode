//! <https://leetcode.com/problems/valid-sudoku/>
use std::collections::HashSet;
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..board.len() {
        let mut hashset = HashSet::new();
        for j in 0..board[i].len() {
            if board[j][i] != '.' {
                if hashset.contains(&board[j][i]) {
                    return false;
                }
                hashset.insert(board[j][i]);
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            if !has_repetition(
                board[(i * 3)..((i + 1) * 3)]
                    .iter()
                    .map(|row| &row[(j * 3)..((j + 1) * 3)])
                    .flatten(),
            ) {
                return false;
            }
        }
    }
    board.iter().all(|row| has_repetition(row.iter()))
}

fn has_repetition<'a, I>(iter: I) -> bool
where
    I: Iterator<Item = &'a char> + Clone,
{
    let iter = iter.filter(|&&ch| ch != '.');
    iter.clone().collect::<HashSet<&char>>().len() == iter.count()
}

fn main() {
    assert!(is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]));
    assert!(!is_valid_sudoku(vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]));
}
