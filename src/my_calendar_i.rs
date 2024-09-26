//! <https://leetcode.com/problems/my-calendar-i/>
struct MyCalendar {
    events: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Self { events: Vec::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self
            .events
            .iter()
            .all(|event| event.0 >= end || event.1 <= start)
        {
            self.events.push((start, end));
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let mut cal = MyCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(15, 25));
        assert!(cal.book(20, 30));
    }
}
