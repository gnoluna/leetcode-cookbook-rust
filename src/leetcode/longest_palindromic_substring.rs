use std::str;

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Solution::seach_palindrome_iterative(s.as_str())
    }

    fn seach_palindrome_iterative(s: &str) -> String {
        let mut max_substring: &str = "";
        let mut max_length: usize = 0;

        for center_idx in 0..s.len() {
            let palindromes = [
                Solution::search_odd(s, center_idx),
                Solution::search_even(s, center_idx),
            ];
            for _palindrome in palindromes {
                match _palindrome {
                    Some(_substr) => {
                        if max_length < _substr.len() {
                            max_length = _substr.len();
                            max_substring = _substr;
                        }
                    }
                    None => {}
                }
            }
        }

        max_substring.to_string()
    }

    fn search_even(s: &str, center_idx: usize) -> Option<&str> {
        if center_idx == s.len() - 1 {
            Some(&s[center_idx..center_idx + 1])
        } else if s.as_bytes()[center_idx] == s.as_bytes()[center_idx + 1] {
            let range = if center_idx >= (s.len() / 2) {
                s.len() - 2 - center_idx
            } else {
                center_idx
            };
            let center_idx = center_idx as f32 + 0.5;
            let window = Solution::search_in_range(s, center_idx, range);
            Some(
                &s[(center_idx - window as f32).floor() as usize
                    ..(center_idx + window as f32).floor() as usize + 2],
            )
        } else {
            None
        }
    }

    fn search_odd(s: &str, center_idx: usize) -> Option<&str> {
        let range = if center_idx >= (s.len() / 2) {
            s.len() - 1 - center_idx
        } else {
            center_idx
        };
        let window = Solution::search_in_range(s, center_idx as f32, range);
        Some(&s[center_idx - window..center_idx + window + 1])
    }

    fn search_in_range(s: &str, center_idx: f32, range: usize) -> usize {
        let mut window = 0;
        for i in 0..range + 1 {
            if s.as_bytes()[(center_idx - i as f32).floor() as usize]
                == s.as_bytes()[(center_idx + i as f32).ceil() as usize]
            {
                window = i;
            } else {
                break;
            }
        }
        window
    }

    // recursive solution: too slow
    #[warn(dead_code)]
    fn search_palindrome_recursive(s: &str, start: usize, end: usize) -> Option<(usize, usize)> {
        let mut longest: Option<(usize, usize)> = None;
        if s.as_bytes()[start] == s.as_bytes()[end] {
            if (start + 1 == end) | (start == end) {
                return Some((start, end));
            }
            match Solution::search_palindrome_recursive(&s, start + 1, end - 1) {
                Some((_start, _end)) => {
                    if _start == start + 1 && _end == end - 1 {
                        return Some((start, end));
                    } else {
                        longest = Some((_start, _end));
                    }
                }
                None => {}
            };
        }
        for result in vec![
            Solution::search_palindrome_recursive(s, start + 1, end),
            Solution::search_palindrome_recursive(s, start, end - 1),
        ] {
            match (result, longest) {
                (Some((_start, _end)), Some((_longest_start, _longest_end))) => {
                    if _end - start > _longest_end - _longest_start {
                        longest = result
                    }
                }
                (Some(_), None) => {
                    longest = result;
                }
                _ => {}
            }
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::longest_palindromic_substring::Solution;

    #[test]
    fn case1() {
        let result = Solution::longest_palindrome("babad".to_string());
        assert_eq!(result, "bab".to_string());
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn timeout_case() {
        assert_eq!(
            Solution::longest_palindrome("abbcccbbbcaaccbababcbcabca".to_string()),
            "bbcccbb".to_string()
        )
    }

    #[test]
    fn len_2_case() {
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        )
    }

    #[test]
    fn ccc_case() {
        assert_eq!(
            Solution::longest_palindrome("ccc".to_string()),
            "ccc".to_string()
        )
    }
}
