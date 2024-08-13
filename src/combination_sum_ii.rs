//! <https://leetcode.com/problems/combination-sum-ii/>
fn add_combination(
    candidates: &[i32],
    combinations: &mut Vec<Vec<i32>>,
    mut combination: Vec<i32>,
    mut target: i32,
) {
    if let Some(&first) = candidates.first() {
        if let Some(index) = candidates
            .iter()
            .enumerate()
            .find_map(|(index, &candiate)| if candiate == first { None } else { Some(index) })
        {
            add_combination(
                &candidates[index..],
                combinations,
                combination.clone(),
                target,
            );
        }
        if first <= target {
            target -= first;
            combination.push(first);
            if target == 0 {
                combinations.push(combination.to_vec());
            } else {
                add_combination(&candidates[1..], combinations, combination, target);
            }
        }
    }
}

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
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
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]),
        );
    }

    #[test]
    fn multiple_repeated_numbers() {
        assert_eq!(
            combination_sum2(vec![2, 5, 2, 1, 2], 5)
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([vec![1, 2, 2], vec![5]]),
        );
    }
}
