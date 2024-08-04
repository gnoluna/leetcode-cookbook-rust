pub struct Solution;

impl Solution {
    pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64 {
        let mut horizontal_cut = horizontal_cut;
        let mut vertical_cut = vertical_cut;

        horizontal_cut.sort_unstable();
        vertical_cut.sort_unstable();

        let mut res: i64 = 0;
        let mut h_chunks: i64 = 1;
        let mut v_chunks: i64 = 1;
        let mut h_index: i32 = horizontal_cut.len() as i32 - 1;
        let mut v_index: i32 = vertical_cut.len() as i32 - 1;
        loop {
            if h_index < 0 && v_index < 0 {
                break;
            } else if h_index < 0 && v_index >= 0 {
                res += ((vertical_cut[..v_index as usize + 1].iter().sum::<i32>() as i64)
                    * h_chunks) as i64;
                break;
            } else if v_index < 0 && h_index >= 0 {
                res += ((horizontal_cut[..h_index as usize + 1].iter().sum::<i32>() as i64)
                    * v_chunks) as i64;
                break;
            } else {
                if horizontal_cut[h_index as usize] > vertical_cut[v_index as usize] {
                    res += (horizontal_cut[h_index as usize] as i64 * v_chunks) as i64;
                    h_chunks += 1;
                    h_index -= 1;
                } else {
                    res += (vertical_cut[v_index as usize] as i64 * h_chunks) as i64;
                    v_chunks += 1;
                    v_index -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::minimum_cost_for_cutting_cake_ii::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
        assert_eq!(Solution::minimum_cost(2, 2, vec![7], vec![4]), 15);
        assert_eq!(
            Solution::minimum_cost(6, 3, vec![2, 3, 2, 3, 1], vec![1, 2]),
            28
        );
    }
}
