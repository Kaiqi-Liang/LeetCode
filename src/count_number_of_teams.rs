//! <https://leetcode.com/problems/count-number-of-teams/>
pub fn num_teams(rating: Vec<i32>) -> i32 {
    (1..rating.len() - 1).fold(0, |count, i| {
        let less_than_left = (0..i).fold(0, |a, j| if rating[j] < rating[i] { a + 1 } else { a });
        let greater_than_right =
            (i + 1..rating.len()).fold(0, |a, j| if rating[j] > rating[i] { a + 1 } else { a });
        count
            + less_than_left * greater_than_right
            + (i - less_than_left) * (rating.len() - 1 - i - greater_than_right)
    }) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(num_teams(vec![2, 5, 3, 4, 1]), 3);
    }

    #[test]
    fn no_team() {
        assert_eq!(num_teams(vec![2, 1, 3]), 0);
    }

    #[test]
    fn increasing_order() {
        assert_eq!(num_teams(vec![1, 2, 3, 4]), 4);
    }
}
