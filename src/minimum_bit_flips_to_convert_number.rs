//! <https://leetcode.com/problems/minimum-bit-flips-to-convert-number/>
pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    (0..i32::BITS)
        .filter(|i| {
            let mask = 1 << i;
            mask & start != mask & goal
        })
        .count() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_to_small() {
        assert_eq!(min_bit_flips(10, 7), 3);
    }

    #[test]
    fn small_to_big() {
        assert_eq!(min_bit_flips(3, 4), 3);
    }
}
