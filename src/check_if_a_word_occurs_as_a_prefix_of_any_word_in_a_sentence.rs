//! <https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/>
pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    sentence
        .split(' ')
        .position(|c| c.starts_with(&search_word))
        .map_or(-1, |index| index as i32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            is_prefix_of_word(String::from("i love eating burger"), String::from("burg")),
            4,
        );
    }

    #[test]
    fn more_than_one_word() {
        assert_eq!(
            is_prefix_of_word(
                String::from("this problem is an easy problem"),
                String::from("pro"),
            ),
            2,
        );
    }

    #[test]
    fn no_such_word() {
        assert_eq!(
            is_prefix_of_word(String::from("i am tired"), String::from("you")),
            -1,
        );
    }
}
