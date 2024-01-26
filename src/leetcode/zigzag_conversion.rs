pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s
        }
        let mut rows: Vec<Vec<char>> = vec![Vec::new(); num_rows as usize];
        let mut up: bool = false;
        let mut cur_row: usize = 0;
        for c in s.chars().into_iter() {
            rows[cur_row].push(c);
            if up {
                cur_row -= 1;
                if cur_row == 0 {
                    up = false
                }
            } else {
                cur_row += 1;
                if cur_row == (num_rows - 1) as usize {
                    up = true
                }
            };
        }
        rows.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::zigzag_conversion::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn very_short() {
        assert_eq!(
            Solution::convert("AB".to_string(), 1),
            "AB".to_string()
        );
    }
}
