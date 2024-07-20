pub struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut res: String = String::new();
        let mut last: Option<char> = None;
        let mut chars = s.chars();
        loop {
            match (chars.next(), last) {
                (Some(c), Some(l)) => {
                    let (c_digit, l_digit) = (c.to_digit(10).unwrap(), l.to_digit(10).unwrap());
                    if (c_digit % 2 != l_digit % 2) || c_digit >= l_digit {
                        res.push(l);
                        last = Some(c);
                    } else {
                        res.push(c);
                        res.push(l);
                        break;
                    }
                }
                (Some(c), None) => {
                    last = Some(c);
                }
                (None, Some(l)) => {
                    res.push(l);
                    last = None;
                    break;
                }
                (None, None) => {
                    break;
                }
            }
        }
        res.push_str(chars.as_str());
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::lexicographically_smallest_string::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::get_smallest_string("45320".to_string()),
            "43520".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::get_smallest_string("001".to_string()),
            "001".to_string()
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::get_smallest_string("131".to_string()),
            "113".to_string()
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::get_smallest_string("220".to_string()),
            "202".to_string()
        );
    }
}
