pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let (mut iter_1,mut iter_2) = (nums1.into_iter(), nums2.into_iter());
        let (mut next_1, mut next_2) = (iter_1.next(), iter_2.next());
        let (mut last, mut cur) = (0, 0);
        let mut idx = 0;
        loop {
            match (next_1, next_2) {
                (Some(v1), Some(v2)) => {
                    last = cur;
                    if v1 < v2 {
                        cur = v1;
                        next_1 = iter_1.next();
                    } else {
                        cur = v2;
                        next_2 = iter_2.next();
                    }
                }
                (Some(v), None) => {
                    last = cur;
                    cur = v;
                    next_1 = iter_1.next();
                }
                (None, Some(v)) => {
                    last = cur;
                    cur = v;
                    next_2 = iter_2.next();
                }
                (None, None) => panic!(),
                
            }
            idx += 1;
            if idx == total / 2 + 1 {
                if total % 2 == 0 {
                    return (cur + last) as f64 / 2.0;
                } else {
                    return cur as f64
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::median_two_sorted_arrays::Solution;

    #[test]
    fn case1() {
        let num1 = vec![1, 3];
        let num2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(num1, num2), 2.0);
    }

    #[test]
    fn case2() {
        let num1 = vec![1, 2];
        let num2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(num1, num2), 2.5);
    }

    #[test]
    fn case3() {
        let num1 = vec![];
        let num2 = vec![2, 3];
        assert_eq!(Solution::find_median_sorted_arrays(num1, num2), 2.5);
    }
}
