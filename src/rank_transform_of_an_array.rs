//! <https://leetcode.com/problems/rank-transform-of-an-array/>
use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
};
pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let sorted_arr = BTreeSet::from_iter(arr.clone().into_iter().map(Reverse));
    let number_to_index =
        sorted_arr
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut a, (index, number)| {
                a.insert(number, sorted_arr.len() - index);
                a
            });
    arr.into_iter()
        .map(|number| number_to_index[&Reverse(number)] as _)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_output() {
        assert_eq!(array_rank_transform(vec![4, 1, 2, 3]), vec![4, 1, 2, 3]);
    }

    #[test]
    fn all_the_same() {
        assert_eq!(array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
    }

    #[test]
    fn happy_path() {
        assert_eq!(
            array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3],
        );
    }
}
