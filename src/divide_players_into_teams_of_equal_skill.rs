//! <https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/>
use std::collections::HashSet;
pub fn divide_players(mut skill: Vec<i32>) -> i64 {
    skill.sort();
    let teams = skill
        .iter()
        .skip(skill.len() / 2)
        .zip(skill.iter().take(skill.len() / 2).rev())
        .map(|(&a, &b)| (a as i64, b as i64))
        .collect::<Vec<_>>();
    if HashSet::<i64>::from_iter(teams.iter().map(|(a, b)| a + b)).len() == 1 {
        teams.into_iter().map(|(a, b)| a * b).sum()
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
    }

    #[test]
    fn only_one_team() {
        assert_eq!(divide_players(vec![3, 4]), 12);
    }

    #[test]
    fn cannot_divide_into_teams() {
        assert_eq!(divide_players(vec![1, 1, 2, 3]), -1);
    }
}
