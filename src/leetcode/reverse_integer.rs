pub struct Solution;

impl Solution {
    pub fn reverse_integer(num: i32) -> i32 {
        let sign: i8 = if (num > 0) { 1 } else { -1 };
        let mut number: i32 = num * sign as i32;
        let mut rev: i64 = 0;
        while number > 0 {
            let digit: i8 = (number % 10).try_into().unwrap();
            number = number / 10;
            rev = rev * 10 + digit as i64;
        }
        rev = rev * sign as i64;
        if rev < i32::MAX as i64 && rev > i32::MIN as i64 {
            rev as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::reverse_integer::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::reverse_integer(1), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::reverse_integer(123), 321);
    }
    #[test]
    fn case3() {
        assert_eq!(Solution::reverse_integer(120), 21);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::reverse_integer(-123), -321);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::reverse_integer(1534236469), 0);
    }
}
