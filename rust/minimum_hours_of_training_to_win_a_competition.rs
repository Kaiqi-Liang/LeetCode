//! <https://leetcode.com/problems/minimum-hours-of-training-to-win-a-competition/>
pub fn min_number_of_hours(
    initial_energy: i32,
    mut initial_experience: i32,
    energy: Vec<i32>,
    experience: Vec<i32>,
) -> i32 {
    let mut days = 0;
    for xp in experience {
        let diff = 0.max(xp - initial_experience + 1);
        initial_experience += xp + diff;
        days += diff;
    }
    days + 0.max(energy.into_iter().sum::<i32>() - initial_energy + 1)
}

fn main() {
    assert_eq!(
        min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
        8,
    );
    assert_eq!(min_number_of_hours(2, 4, vec![1], vec![3]), 0);
    assert_eq!(
        min_number_of_hours(1, 1, vec![1, 1, 1, 1], vec![1, 1, 1, 50]),
        51,
    );
    assert_eq!(
        min_number_of_hours(
            100,
            100,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 1, 2, 3, 1, 2, 10],
        ),
        0,
    );
}
