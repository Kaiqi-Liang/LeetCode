//! <https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/>
use std::collections::HashMap;
pub fn minimum_pushes(word: String) -> i32 {
    let mut counter = counter!(word.chars()).into_iter().map(|(_, count)| count).collect::<Vec<_>>();
    counter.sort_by(|a, b| b.cmp(a));
	counter.into_iter().enumerate().fold(0, |acc, (i, count)| {
		acc + (i as i32 / 8 + 1) * count
	})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_letters_fewer_than_8() {
        assert_eq!(minimum_pushes(String::from("abcde")), 5);
    }

    #[test]
    fn repeated_letters_fewer_than_8() {
        assert_eq!(minimum_pushes(String::from("xyzxyzxyzxyz")), 12);
    }

    #[test]
    fn repeated_letters_more_than_8() {
        assert_eq!(minimum_pushes(String::from("aabbccddeeffgghhiiiiii")), 24);
    }
}
