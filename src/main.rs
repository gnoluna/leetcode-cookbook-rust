use crate::leetcode::add_two_numbers::ListNode;

mod leetcode;

fn main() {
    let res = leetcode::two_sum::Solution::two_sum(vec![1, 1, 3], 2);
    println!("001. Two sum: {:?}", &res);

    let res = leetcode::add_two_numbers::Solution::add_two_numbers(
        Some(Box::new(ListNode::from_number(342))),
        Some(Box::new(ListNode::from_number(465))),
    );
    println!("002. Add two numbers: {:?}", &res);

    let res = leetcode::longest_substring_wo_repeating_chars::Solution::length_of_longest_substring(
        "abcabcbb".to_string(),
    );
    println!(
        "003. Longest substring without repeating characters: {:?}",
        &res
    );

    let res = leetcode::median_two_sorted_arrays::Solution::find_median_sorted_arrays(
        vec![1, 3],
        vec![2],
    );
    println!("004. Median of Two Sorted Arrays: {:?}", &res);
}
