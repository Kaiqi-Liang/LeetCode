//! <https://leetcode.com/problems/sort-the-people/>
pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut names_and_heights = names.into_iter().zip(heights).collect::<Vec<_>>();
    names_and_heights.sort_by(|(_, height1), (_, height2)| height2.cmp(height1));
    names_and_heights
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            sort_people(
                vec![
                    String::from("Mary"),
                    String::from("John"),
                    String::from("Emma"),
                ],
                vec![180, 165, 170],
            ),
            vec![
                String::from("Mary"),
                String::from("Emma"),
                String::from("John"),
            ],
        );
    }

    #[test]
    fn same_name() {
        assert_eq!(
            sort_people(
                vec![
                    String::from("Alice"),
                    String::from("Bob"),
                    String::from("Bob"),
                ],
                vec![155, 185, 150],
            ),
            vec![
                String::from("Bob"),
                String::from("Alice"),
                String::from("Bob"),
            ],
        );
    }
}
