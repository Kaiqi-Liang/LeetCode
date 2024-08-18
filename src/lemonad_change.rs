//! <https://leetcode.com/problems/lemonade-change/>
use std::collections::HashMap;
macro_rules! make_change {
    ($change:ident, $changes:ident, $note:expr) => {
        if $change >= $note {
            if let Some(amount) = $changes.get_mut(&$note) {
                if *amount == 0 {
                    $changes.remove(&$note);
                    false
                } else {
                    *amount -= 1;
                    $change -= $note;
                    true
                }
            } else {
                false
            }
        } else {
            false
        }
    };
}
pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut changes = HashMap::new();
    for bill in bills {
        let mut change = bill - 5;
        while change > 0 {
            if !make_change!(change, changes, 10) && !make_change!(change, changes, 5) {
                return false;
            }
        }
        if bill < 20 {
            *changes.entry(bill).or_insert(0) += 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert!(lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn not_enough_fives() {
        assert!(!lemonade_change(vec![5, 5, 10, 10, 20]));
    }

    #[test]
    fn first_one_not_five() {
        assert!(!lemonade_change(vec![10, 20]));
    }

    #[test]
    fn too_many_twenties() {
        assert!(!lemonade_change(vec![5, 5, 5, 10, 5, 5, 10, 20, 20, 20]));
    }
}
