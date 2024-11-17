//! <https://leetcode.com/problems/daily-temperatures/>
/// `O(n^2)` time complexity
mod nested_loop {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            for j in i..temperatures.len() {
                if temperatures[j] > temperatures[i] {
                    answer[i] = (j - i) as _;
                    break;
                }
            }
        }
        answer
    }
}

/// `O(n^2)` time complexity
mod memorisation {
    const MAX_TEMP: usize = 100;
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut indices = vec![Vec::<usize>::new(); MAX_TEMP];
        let mut answer = vec![0; temperatures.len()];

        for (i, &temp) in temperatures.iter().enumerate() {
            indices[temp as usize - 1].push(i);
        }

        for (i, temp) in temperatures.into_iter().enumerate() {
            let mut min = usize::MAX;
            for future in temp + 1..=MAX_TEMP as _ {
                for &index in indices[future as usize - 1].iter() {
                    if index > i && index < min {
                        min = index;
                        break;
                    }
                }
            }
            if min < usize::MAX {
                answer[i] = (min - i) as i32;
            }
        }
        answer
    }
}

/// `O(n)` time complexity
mod backwards {
    const MAX_TEMP: usize = 100;
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0; temperatures.len()];
        let mut indices = vec![usize::MAX; MAX_TEMP];
        indices[(*temperatures.last().expect("temperatures.len() >= 1") - 1) as usize] =
            temperatures.len() - 1;

        for (i, &temp) in temperatures.iter().enumerate().rev().skip(1) {
            indices[(temp - 1) as usize] = i;
            if temp < MAX_TEMP as _ {
                if let Some(&temp) = indices[temp as usize..MAX_TEMP].iter().min() {
                    if temp != usize::MAX {
                        answer[i] = (temp - i) as i32;
                    }
                }
            }
        }
        answer
    }
}

/// `O(n)` time complexity
mod monotonic_stack {
    use std::collections::VecDeque;
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
        let mut res = vec![0; temperatures.len()];
        for (curr_index, curr_temp) in temperatures.into_iter().enumerate() {
            while let Some(&(prev_index, prev_temp)) = stack.back() {
                if curr_temp > prev_temp {
                    stack.pop_back();
                    res[prev_index] = (curr_index - prev_index) as i32;
                } else {
                    break;
                }
            }
            stack.push_back((curr_index, curr_temp));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nested_loop() {
        assert_eq!(
            nested_loop::daily_temperatures(vec![95, 99, 95, 93, 96, 100, 97]),
            vec![1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            nested_loop::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            nested_loop::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0],
        );
        assert_eq!(
            nested_loop::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0],
        );
        assert_eq!(
            nested_loop::daily_temperatures(vec![33, 32, 31]),
            vec![0, 0, 0],
        );
    }

    #[test]
    fn memorisation() {
        assert_eq!(
            memorisation::daily_temperatures(vec![95, 99, 95, 93, 96, 100, 97]),
            vec![1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            memorisation::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            memorisation::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0],
        );
        assert_eq!(
            memorisation::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0],
        );
        assert_eq!(
            memorisation::daily_temperatures(vec![33, 32, 31]),
            vec![0, 0, 0],
        );
    }

    #[test]
    fn backwards() {
        assert_eq!(
            backwards::daily_temperatures(vec![95, 99, 95, 93, 96, 100, 97]),
            vec![1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            backwards::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            backwards::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0],
        );
        assert_eq!(
            backwards::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0],
        );
        assert_eq!(
            backwards::daily_temperatures(vec![33, 32, 31]),
            vec![0, 0, 0],
        );
    }

    #[test]
    fn monotonic_stack() {
        assert_eq!(
            monotonic_stack::daily_temperatures(vec![95, 99, 95, 93, 96, 100, 97]),
            vec![1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            monotonic_stack::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0],
        );
        assert_eq!(
            monotonic_stack::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0],
        );
        assert_eq!(
            monotonic_stack::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0],
        );
        assert_eq!(
            monotonic_stack::daily_temperatures(vec![33, 32, 31]),
            vec![0, 0, 0],
        );
    }
}
