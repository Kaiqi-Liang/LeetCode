//! <https://leetcode.com/problems/water-bottles/>
pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut res = num_bottles;
    let mut num_bottles = num_bottles;
    loop {
        let exchanged_bottles = num_bottles / num_exchange;
        if exchanged_bottles == 0 {
            break;
        }
        res += exchanged_bottles;
        num_bottles = exchanged_bottles + num_bottles % num_exchange;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisble() {
        assert_eq!(num_water_bottles(9, 3), 13);
    }

    #[test]
    fn not_divisble() {
        assert_eq!(num_water_bottles(15, 4), 19);
    }
}
