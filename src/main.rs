

mod leetcode;

fn main() {
    let res = leetcode::two_sum::Solution::two_sum(vec![1, 1, 3], 2);
    println!("{:?}", &res);
}
