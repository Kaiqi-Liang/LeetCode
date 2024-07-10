//! <https://leetcode.com/problems/average-waiting-time/>
pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut curr_time: u64 = 0;
    let mut wait_time: u64 = 0;
    let num_customers = customers.len() as f64;
    for customer in customers {
        if let [arrival, preparation_time] = customer[..] {
            let arrival = arrival as u64;
            let preparation_time = preparation_time as u64;
            if curr_time < arrival {
                curr_time = arrival;
            }
            curr_time += preparation_time;
            wait_time += curr_time - arrival;
        } else {
            panic!("The vector has more than 2 elements");
        }
    }
    wait_time as f64 / num_customers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap() {
        assert_eq!(
            average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]),
            5.0
        );
    }

    #[test]
    fn no_overlap() {
        assert_eq!(
            average_waiting_time(vec![vec![0, 2], vec![2, 4], vec![10, 3]]),
            3.0
        );
    }

    #[test]
    fn arrive_the_same_time() {
        assert_eq!(
            average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]),
            3.25
        );
    }
}
