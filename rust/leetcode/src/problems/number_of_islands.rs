pub struct Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
        let mut count = 0;
        let grid_y = grid.len();
        if grid_y == 0 {
            return 0
        }
        let grid_x = grid[0].len();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !visited.contains(&(i,j)) && grid[i][j] == '1' {
                    count += 1;
                    to_visit.push_back((i,j));
                    while let Some((i, j)) = to_visit.pop_front() {
                        let left = i.checked_sub(1).map(|i| (i, j));
                        let right = i.checked_add(1).filter(|sum| sum < &grid_y).map(|i| (i, j));
                        let top = j.checked_sub(1).map(|j| (i, j));
                        let bot = j.checked_add(1).filter(|sum| sum < &grid_x).map(|j| (i,j));
                        let neighbors = [left, top, right, bot];
                        for (i,j) in neighbors.into_iter().flatten() {
                            if !visited.contains(&(i,j)) && grid[i][j] == '1' {
                                to_visit.push_back((i,j));
                                visited.insert((i,j));
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn counts_one_connected_island() {
        let input = grid(&["11110", "11010", "11000", "00000"]);

        assert_eq!(Solution::num_islands(input), 1);
    }

    #[test]
    fn counts_three_separate_islands() {
        let input = grid(&["11000", "11000", "00100", "00011"]);

        assert_eq!(Solution::num_islands(input), 3);
    }

    #[test]
    fn returns_zero_for_empty_grid() {
        assert_eq!(Solution::num_islands(vec![]), 0);
    }

    #[test]
    fn returns_zero_for_only_water() {
        let input = grid(&["000", "000", "000"]);

        assert_eq!(Solution::num_islands(input), 0);
    }

    #[test]
    fn returns_one_for_only_land() {
        let input = grid(&["111", "111", "111"]);

        assert_eq!(Solution::num_islands(input), 1);
    }

    #[test]
    fn does_not_connect_diagonal_land() {
        let input = grid(&["10", "01"]);

        assert_eq!(Solution::num_islands(input), 2);
    }

    #[test]
    fn counts_single_land_cell() {
        let input = grid(&["1"]);

        assert_eq!(Solution::num_islands(input), 1);
    }

    #[test]
    fn ignores_single_water_cell() {
        let input = grid(&["0"]);

        assert_eq!(Solution::num_islands(input), 0);
    }

    #[test]
    fn returns_zero_for_grid_with_empty_row() {
        let input = grid(&[""]);

        assert_eq!(Solution::num_islands(input), 0);
    }

    #[test]
    fn counts_islands_in_single_row() {
        let input = grid(&["101101"]);

        assert_eq!(Solution::num_islands(input), 3);
    }

    #[test]
    fn counts_islands_in_single_column() {
        let input = grid(&["1", "0", "1", "1", "0", "1"]);

        assert_eq!(Solution::num_islands(input), 3);
    }

    #[test]
    fn counts_checkerboard_land_as_separate_islands() {
        let input = grid(&["101", "010", "101"]);

        assert_eq!(Solution::num_islands(input), 5);
    }

    #[test]
    fn counts_ring_around_water_as_one_island() {
        let input = grid(&["111", "101", "111"]);

        assert_eq!(Solution::num_islands(input), 1);
    }

    #[test]
    fn counts_multiple_irregular_islands() {
        let input = grid(&["11100", "10001", "00101", "00110"]);

        assert_eq!(Solution::num_islands(input), 3);
    }
}
