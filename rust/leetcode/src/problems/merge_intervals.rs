use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut intervals = intervals.clone();
        intervals.sort_by_key(|item| item.0);

        let mut ret = vec![intervals[0]];
        let mut idx = 0;

        for interval in &intervals[1..] {
            let right = &mut ret[idx].1;
            if *right >= interval.0 {
                *right = max(*right, interval.1);
                continue;
            }

            ret.push(*interval);
            idx += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_overlapping_intervals() {
        assert_eq!(
            Solution::merge(vec![(1, 3), (2, 6), (8, 10), (15, 18)]),
            vec![(1, 6), (8, 10), (15, 18)]
        );
    }

    #[test]
    fn merges_touching_intervals() {
        assert_eq!(Solution::merge(vec![(1, 4), (4, 5)]), vec![(1, 5)]);
    }

    #[test]
    fn sorts_before_merging() {
        assert_eq!(Solution::merge(vec![(4, 7), (1, 4)]), vec![(1, 7)]);
    }

    #[test]
    fn merges_intervals_contained_inside_larger_interval() {
        assert_eq!(
            Solution::merge(vec![(1, 10), (2, 3), (4, 5)]),
            vec![(1, 10)]
        );
    }

    #[test]
    fn leaves_disjoint_intervals_separate() {
        assert_eq!(
            Solution::merge(vec![(1, 2), (4, 5), (8, 9)]),
            vec![(1, 2), (4, 5), (8, 9)]
        );
    }

    #[test]
    fn handles_single_interval() {
        assert_eq!(Solution::merge(vec![(3, 6)]), vec![(3, 6)]);
    }

    #[test]
    fn handles_duplicate_intervals() {
        assert_eq!(
            Solution::merge(vec![(2, 4), (2, 4), (2, 4)]),
            vec![(2, 4)]
        );
    }

    #[test]
    fn handles_negative_values() {
        assert_eq!(
            Solution::merge(vec![(-10, -5), (-7, 0), (2, 3)]),
            vec![(-10, 0), (2, 3)]
        );
    }
}
