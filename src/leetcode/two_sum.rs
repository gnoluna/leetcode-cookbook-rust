use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        let mut res: Vec<i32> = vec![0, 0];
        for (i, n) in nums.iter().enumerate() {
            match seen.get(&(target - n)) {
                Some(idx) => {
                    res = vec![*idx, (i as i32)];
                }
                None => {
                    seen.insert(n, i as i32);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::two_sum::Solution;

    #[test]
    fn case1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
