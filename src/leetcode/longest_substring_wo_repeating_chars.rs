use std::{cmp, collections::HashMap};
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut max_length = 0;
        let mut last_idx = 0;
        for (i, c) in s.chars().enumerate() {
            match map.get(&c) {
                Some(idx) => {
                    if max_length == 0 {
                        max_length = i;
                    }
                    let length = i - idx;
                    if length > max_length {
                        max_length = length;
                    }
                    map.insert(c, i);
                    last_idx = i;
                }
                None => {
                    map.insert(c, i);
                }
            }
        }
        cmp::max(s.len() - last_idx, max_length) as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::leetcode::longest_substring_wo_repeating_chars::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }
    #[test]
    fn case2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }
    #[test]
    fn case3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }

    #[test]
    fn case6() {
        assert_eq!(Solution::length_of_longest_substring("cdd".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
    }
}
