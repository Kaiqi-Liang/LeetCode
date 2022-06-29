//! <https://leetcode.com/problems/valid-parentheses/>
use std::collections::HashMap;
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::<char>::new();
    let matching_parentheses = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    for ch in s.chars() {
        if !match ch {
            '(' | '[' | '{' => {
                stack.push(ch);
                true
            }
            ')' | ']' | '}' => match stack.pop() {
                Some(parenthesis) => ch == *matching_parentheses.get(&parenthesis).unwrap(),
                None => false,
            },
            _ => true,
        } {
            return false;
        }
    }
    stack.is_empty()
}

fn main() {
    assert!(is_valid(String::from("()")));
    assert!(is_valid(String::from("()[]{}")));
    assert!(!is_valid(String::from("(]")));
}
