//! <https://leetcode.com/problems/combination-sum/>
fn add_combination(
    candidates: &[i32],
    combinations: &mut Vec<Vec<i32>>,
    mut combination: Vec<i32>,
    mut target: i32,
) {
    if let Some(&first) = candidates.first() {
        add_combination(&candidates[1..], combinations, combination.clone(), target);
        if first <= target {
            target -= first;
            combination.push(first);
            if target == 0 {
                combinations.push(combination.to_vec());
            } else {
                add_combination(candidates, combinations, combination, target);
            }
        }
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut combinations = Vec::new();
    add_combination(&candidates, &mut combinations, Vec::new(), target);
    combinations
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn happy_path() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7)
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([vec![2, 2, 3], vec![7]]),
        );
    }

    #[test]
    fn higher_target() {
        assert_eq!(
            combination_sum(vec![3, 5, 8], 11)
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([vec![3, 3, 5], vec![3, 8]]),
        );
    }

    #[test]
    fn non_increasing_order() {
        assert_eq!(
            combination_sum(vec![3, 7, 6, 2], 7)
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([vec![3, 2, 2], vec![7]]),
        );
    }

    #[test]
    fn all_same_number() {
        assert_eq!(
            combination_sum(vec![2, 3, 5], 8)
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]),
        );
    }

    #[test]
    fn one_number() {
        assert_eq!(combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }
}
