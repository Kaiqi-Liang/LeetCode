//! <https://leetcode.com/problems/number-complement/>
pub fn find_complement(num: i32) -> i32 {
    for i in (0..i32::BITS).rev() {
        let mask = 1 << i;
        if mask & num > 0 {
            return !num & (i32::MAX >> (i32::BITS - i - 2));
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(find_complement(5), 2);
    }

    #[test]
    fn all_ones() {
        assert_eq!(find_complement(7), 0);
    }

    #[test]
    fn zero() {
        assert_eq!(find_complement(0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(find_complement(1), 0);
    }
}
