//! <https://leetcode.com/problems/guess-number-higher-or-lower/>
fn guess(num: i64, pick: i64) -> i32 {
    match pick.cmp(&num) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// `O(log_2{n})` time complexity
mod binary_search {
    use super::guess;
    pub fn guess_number(n: i64, pick: i64) -> i64 {
        let mut low = 1;
        let mut high = n;
        while low < high {
            let num = (low + high) / 2;
            let result = guess(num, pick);
            if result == 1 {
                low = num + 1;
            } else if result == -1 {
                high = num - 1;
            } else {
                return num;
            }
        }
        low
    }
}

/// `O(log_3{n})` time complexity
mod ternary_search {
    use super::guess;
    pub fn guess_number(n: i64, pick: i64) -> i64 {
        let mut low = 1;
        let mut high = n;
        while low < high {
            let mid1 = low + (low - high) / 3;
            let mid2 = high + (low - high) / 3;
            let res1 = guess(mid1, pick);
            let res2 = guess(mid2, pick);
            if res1 == 0 {
                return mid1;
            } else if res2 == 0 {
                return mid2;
            } else if res1 == -1 {
                high = mid1 - 1;
            } else if res2 == 1 {
                low = mid2 + 1;
            } else {
                low = mid1 + 1;
                high = mid2 - 1;
            }
        }
        low
    }
}

fn main() {
    assert_eq!(
        binary_search::guess_number(10, 6),
        ternary_search::guess_number(10, 6)
    );
    assert_eq!(
        binary_search::guess_number(1, 1),
        ternary_search::guess_number(1, 1)
    );
    assert_eq!(
        binary_search::guess_number(2, 1),
        ternary_search::guess_number(2, 1)
    );
    assert_eq!(
        binary_search::guess_number(11702766719, 2126753390),
        ternary_search::guess_number(11702766719, 2126753390)
    );
}
