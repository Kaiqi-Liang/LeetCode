//! <https://leetcode.com/problems/pass-the-pillow/>
pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let n = n - 1;
    let forward = (time / n) % 2 == 0;
    let offset = time % n;
    (if forward { offset } else { n - offset }) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_pass() {
        assert_eq!(pass_the_pillow(3, 2), 3);
    }

    #[test]
    fn two_passes() {
        assert_eq!(pass_the_pillow(4, 5), 2);
    }

    #[test]
    fn three_passes() {
        assert_eq!(pass_the_pillow(5, 9), 2);
    }

    #[test]
    fn four_passes() {
        assert_eq!(pass_the_pillow(4, 11), 2);
    }
}
