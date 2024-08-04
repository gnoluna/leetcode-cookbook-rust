pub struct Solution;

impl Solution {
    pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
        let res = if m == 1 {
            vertical_cut.iter().sum()
        } else if n == 1 {
            horizontal_cut.iter().sum()
        } else {
            let max_h = horizontal_cut
                .iter()
                .enumerate()
                .max_by_key(|&(_, value)| value);
            let max_v = vertical_cut
                .iter()
                .enumerate()
                .max_by_key(|&(_, value)| value);
            if let (Some((pos_h, v_h)), Some((pos_v, v_v))) = (max_h, max_v) {
                if v_h < v_v {
                    // cut v
                    *v_v + Self::minimum_cost(
                        m,
                        pos_v as i32 + 1,
                        horizontal_cut.to_owned(),
                        vertical_cut[..pos_v].to_vec(),
                    ) + Self::minimum_cost(
                        m,
                        n - pos_v as i32 - 1,
                        horizontal_cut.to_owned(),
                        vertical_cut[pos_v + 1..].to_vec(),
                    )
                } else {
                    // cut h
                    *v_h + Self::minimum_cost(
                        pos_h as i32 + 1,
                        n,
                        horizontal_cut[..pos_h].to_vec(),
                        vertical_cut.to_owned(),
                    ) + Self::minimum_cost(
                        m - pos_h as i32 - 1,
                        n,
                        horizontal_cut[pos_h + 1..].to_vec(),
                        vertical_cut.to_owned(),
                    )
                }
            } else {
                0
            }
        };
        // println!(
        //     "{},{}={}: {:?} {:?}",
        //     &m, &n, &res, &horizontal_cut, &vertical_cut
        // );
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::minimum_cost_for_cutting_cake::Solution;

    #[test]
    fn case1() {
        // assert_eq!(Solution::minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
        // assert_eq!(Solution::minimum_cost(2, 2, vec![7], vec![4]), 15);
        assert_eq!(
            Solution::minimum_cost(6, 3, vec![2, 3, 2, 3, 1], vec![1, 2]),
            28
        );
    }
}
