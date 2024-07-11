//! <https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/>
use std::collections::VecDeque;
fn reverse(s: String, begin: usize, end: usize) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars[begin..end].reverse();
    chars.into_iter().collect()
}

pub fn reverse_parentheses(s: String) -> String {
    let mut stack = VecDeque::new();
    let mut num_braces = 0;
    let mut res = String::new();
    for (i, ch) in s.char_indices() {
        if ch == '(' {
            stack.push_back(i - num_braces);
            num_braces += 1;
        } else if ch == ')' {
            let last_open = stack.pop_back().expect("All parentheses are balanced");
            res = reverse(res, last_open, i - num_braces);
            num_braces += 1;
        } else {
            res.push(ch);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_set_of_parentheses() {
        assert_eq!(reverse_parentheses(String::from("(abcd)")), "dcba");
    }

    #[test]
    fn even_number() {
        assert_eq!(reverse_parentheses(String::from("(u(love)i)")), "iloveu");
    }

    #[test]
    fn odd_number() {
        assert_eq!(
            reverse_parentheses(String::from("(ed(et(oc))el)")),
            "leetcode"
        );
    }

    #[test]
    fn start_with_a_char() {
        assert_eq!(
            reverse_parentheses(String::from("a(bcdefghijkl(mno)p)q")),
            "apmnolkjihgfedcbq"
        );
    }
}
