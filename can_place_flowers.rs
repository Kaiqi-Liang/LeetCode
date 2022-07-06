//! <https://leetcode.com/problems/can-place-flowers/>
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut zeros = 0;
    let mut count = 0;
    let mut prev = 0;
    for flower in flowerbed {
        if flower == 0 {
            zeros += 1;
        } else {
            count += (zeros - if prev == 1 { 1 } else { 0 }) / 2;
            prev = flower;
            zeros = 0;
        }
    }
    if zeros != 0 {
        count += (zeros + if prev == 0 { 1 } else { 0 }) / 2;
    }
    count >= n
}

fn main() {
    assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    assert!(can_place_flowers(vec![1, 0, 0, 0, 0, 0, 0, 0, 1], 1));
    assert!(can_place_flowers(vec![0, 0, 0, 0, 1], 2));
    assert!(can_place_flowers(vec![0, 0, 0, 0, 0], 3));
    assert!(can_place_flowers(vec![0, 1, 0, 0, 0], 1));
    assert!(!can_place_flowers(vec![0, 1, 0, 0, 0], 2));
    assert!(!can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
    assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    assert!(!can_place_flowers(vec![1], 1));
    assert!(can_place_flowers(vec![0], 1));
}
