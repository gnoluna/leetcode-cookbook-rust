use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut max_length = 0;
        let mut i: usize = 0;
        for (j, c) in s.chars().enumerate() {
            match map.get(&c) {
                Some(idx) => {
                    let _idx = *idx;
                    for x in i.._idx {
                        map.remove_entry(&s.chars().nth(x).unwrap());
                    }
                    i = _idx + 1;
                }
                None => {}
            }
            map.insert(c, j);
            let length = j - i + 1;
            if length > max_length {
                max_length = length;
            }
        }
        max_length as i32
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
