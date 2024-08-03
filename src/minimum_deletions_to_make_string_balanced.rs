//! <https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/>
pub fn minimum_deletions(s: String) -> i32 {
    let mut count_a = s.chars().filter(|&ch| ch == 'a').count();
    let mut count_b = 0;
    s.chars().fold(s.len(), |mut a, c| {
        if c == 'a' {
            count_a -= 1;
        }
        a = a.min(count_a + count_b);
        if c == 'b' {
            count_b += 1;
        }
        a
    }) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_ways() {
        assert_eq!(minimum_deletions(String::from("aababbab")), 2);
    }

    #[test]
    fn delete_from_beginning() {
        assert_eq!(minimum_deletions(String::from("bbaaaaabb")), 2);
    }

    #[test]
    fn delete_last_character() {
        assert_eq!(minimum_deletions(String::from("bbbba")), 1);
    }

    #[test]
    fn delete_middle_one() {
        assert_eq!(minimum_deletions(String::from("aaababbb")), 1);
    }

    #[test]
    fn delete_middle_two() {
        assert_eq!(minimum_deletions(String::from("aaabababbb")), 2);
    }

    #[test]
    fn delete_middle_three_consecutive() {
        assert_eq!(minimum_deletions(String::from("aaababaababbb")), 3);
    }

    #[test]
    fn delete_middle_three_alternate() {
        assert_eq!(minimum_deletions(String::from("aaababababbb")), 3);
    }

    #[test]
    fn counterexample() {
        assert_eq!(
            minimum_deletions(String::from("aaaabbbbaaabbbabababbaaa")),
            9
        );
    }

    #[test]
    fn long() {
        assert_eq!(
            minimum_deletions(String::from(
                "ababaaaabbbbbaaababbbbbbaaabbaababbabbbbaabbbbaabbabbabaabbbababaa"
            )),
            25,
        );
    }
}
