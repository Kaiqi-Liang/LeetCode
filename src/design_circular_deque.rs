//! <https://leetcode.com/problems/design-circular-deque/>
use std::collections::VecDeque;
struct MyCircularDeque {
    deque: VecDeque<i32>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            deque: VecDeque::with_capacity(k as _),
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        check_and_operate(self.is_full(), || {
            self.deque.push_front(value);
        })
    }

    fn insert_last(&mut self, value: i32) -> bool {
        check_and_operate(self.is_full(), || {
            self.deque.push_back(value);
        })
    }

    fn delete_front(&mut self) -> bool {
        check_and_operate(self.is_empty(), || {
            self.deque.pop_front();
        })
    }

    fn delete_last(&mut self) -> bool {
        check_and_operate(self.is_empty(), || {
            self.deque.pop_back();
        })
    }

    fn get_front(&self) -> i32 {
        *self.deque.front().unwrap_or(&-1)
    }

    fn get_rear(&self) -> i32 {
        *self.deque.back().unwrap_or(&-1)
    }

    fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    fn is_full(&self) -> bool {
        self.deque.len() == self.deque.capacity()
    }
}

fn check_and_operate(check: bool, operation: impl FnOnce()) -> bool {
    if check {
        false
    } else {
        operation();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let mut deque = MyCircularDeque::new(3);
        assert!(deque.insert_last(1));
        assert!(deque.insert_last(2));
        assert!(deque.insert_front(3));
        assert!(!deque.insert_front(4));
        assert_eq!(deque.get_rear(), 2);
        assert!(deque.is_full());
        assert!(deque.delete_last());
        assert!(deque.insert_front(4));
        assert_eq!(deque.get_front(), 4);
    }
}
