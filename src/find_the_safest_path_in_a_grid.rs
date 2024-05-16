//! <https://leetcode.com/problems/find-the-safest-path-in-a-grid/>
use crate::coordinate::Coordinate;
use std::collections::{BinaryHeap, HashSet, VecDeque};

pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    let mut dist = vec![vec![0; grid[0].len()]; grid.len()];
    let theives = grid
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, &thief)| {
            if thief == 1 {
                Some(Coordinate {
                    x: i / grid.len(),
                    y: i % grid.len(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut q = VecDeque::from(theives.clone());
    let mut visited: HashSet<Coordinate> = theives.into_iter().collect();
    while !q.is_empty() {
        let curr_coord = q.pop_front().expect("!q.is_empty()");
        for new_coord in curr_coord.neighbours(grid.len(), grid.len()) {
            if !visited.contains(&new_coord) {
                visited.insert(new_coord);
                dist[new_coord.x][new_coord.y] = dist[curr_coord.x][curr_coord.y] + 1;
                q.push_back(new_coord);
            }
        }
    }
    let mut priority_queue =
        BinaryHeap::from([(dist[0][0], dist[0][0], Coordinate { x: 0, y: 0 })]);
    let mut visited = HashSet::from([Coordinate { x: 0, y: 0 }]);
    while !priority_queue.is_empty() {
        let (_, curr_min, curr_coord) = priority_queue.pop().expect("!priority_queue.is_empty()");
        if curr_coord.x == dist.len() - 1 && curr_coord.y == dist.len() - 1 {
            return curr_min;
        }
        for new_coord in curr_coord.neighbours(dist.len(), dist.len()) {
            if !visited.contains(&new_coord) {
                visited.insert(new_coord);
                priority_queue.push((
                    dist[new_coord.x][new_coord.y],
                    curr_min.min(dist[new_coord.x][new_coord.y]),
                    new_coord,
                ));
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_path() {
        assert_eq!(
            maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            0
        );
    }

    #[test]
    fn happy_path() {
        assert_eq!(
            maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
            2
        );
    }
}
