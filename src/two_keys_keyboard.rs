//! <https://leetcode.com/problems/2-keys-keyboard/>
pub fn min_steps(n: i32) -> i32 {
    let n = n as usize;
    let mut opt = vec![usize::MAX; n];
    opt[0] = 0;
    for i in 1..n {
        for j in 1..=(i + 1) / 2 {
            if (i + 1) % j == 0 {
                opt[i] = opt[i].min(opt[j - 1] + (i + 1) / j);
            }
        }
    }
    opt[n - 1] as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get1() {
        assert_eq!(min_steps(1), 0);
    }

    #[test]
    fn get2() {
        assert_eq!(min_steps(2), 2);
    }

    #[test]
    fn get3() {
        assert_eq!(min_steps(3), 3);
    }

    #[test]
    fn get4() {
        assert_eq!(min_steps(4), 4);
    }

    #[test]
    fn get5() {
        assert_eq!(min_steps(5), 5);
    }

    #[test]
    fn get6() {
        assert_eq!(min_steps(6), 5);
    }

    #[test]
    fn get7() {
        assert_eq!(min_steps(7), 7);
    }
}
