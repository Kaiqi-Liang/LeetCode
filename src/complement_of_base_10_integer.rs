//! <https://leetcode.com/problems/complement-of-base-10-integer/>
pub fn bitwise_complement(n: i32) -> i32 {
    for i in (0..i32::BITS).rev() {
        let mask = 1 << i;
        if mask & n > 0 {
            return !n & (i32::MAX >> (i32::BITS - i - 2));
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(bitwise_complement(5), 2);
    }

    #[test]
    fn all_ones() {
        assert_eq!(bitwise_complement(7), 0);
    }

    #[test]
    fn zero() {
        assert_eq!(bitwise_complement(0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(bitwise_complement(1), 1);
    }

    #[test]
    fn ten() {
        assert_eq!(bitwise_complement(10), 5);
    }
}
