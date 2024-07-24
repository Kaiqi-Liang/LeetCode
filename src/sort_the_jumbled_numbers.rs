//! <https://leetcode.com/problems/sort-the-jumbled-numbers/>
pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    let mut mapped_nums = nums
        .into_iter()
        .map(|num| {
            (
                num,
                num.to_string()
                    .chars()
                    .map(|digit| mapping[digit.to_digit(10).unwrap() as usize])
                    .fold(0, |a, c| a * 10 + c),
            )
        })
        .collect::<Vec<_>>();
    mapped_nums.sort_by_key(|num| num.1);
    mapped_nums.into_iter().map(|num| num.0).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
            vec![338, 38, 991]
        );
    }

    #[test]
    fn same_order() {
        assert_eq!(
            sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
            vec![123, 456, 789]
        );
    }
}
