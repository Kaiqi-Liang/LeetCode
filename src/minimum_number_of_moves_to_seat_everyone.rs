//! <https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/>
pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;
    seats.sort();
    students.sort();
    seats
        .into_iter()
        .zip(students)
        .fold(0, |a, c| a + (c.0 - c.1).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(min_moves_to_seat(vec![3, 1, 5], vec![2, 4, 7]), 4);
    }

    #[test]
    fn no_move_needed() {
        assert_eq!(min_moves_to_seat(vec![3, 1, 5], vec![1, 3, 5]), 0);
    }
}
