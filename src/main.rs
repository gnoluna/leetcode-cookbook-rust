use crate::leetcode::add_two_numbers::ListNode;

mod leetcode;

fn main() {
    let res = leetcode::two_sum::Solution::two_sum(vec![1, 1, 3], 2);
    println!("001. Two sum: {:?}", &res);

    let res = leetcode::add_two_numbers::Solution::add_two_numbers(
        Some(Box::new(ListNode::from_number(342))),
        Some(Box::new(ListNode::from_number(465))),
    );
    println!("001. Add two numbers: {:?}", &res);

    let res = leetcode::longest_substring_wo_repeating_chars::Solution::length_of_longest_substring(
        "bbbbbb".to_string()
    );
    println!("001. Longest substring without repeating characters: {:?}", &res);
}
