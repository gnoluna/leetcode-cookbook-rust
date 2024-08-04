use std::collections::HashSet;

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn construct_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut last_node: Option<Box<ListNode>> = None;
    nums.iter().for_each(|n| {
        let mut new_node = ListNode::new(*n);
        new_node.next = last_node.clone();
        last_node = Some(Box::new(new_node));
    });
    last_node
}

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums_set: HashSet<i32> = nums.into_iter().collect();
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut current = dummy.as_mut();

        while let Some(node) = current {
            while let Some(next) = node.next.as_mut() {
                if nums_set.contains(&next.val) {
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
            current = node.next.as_mut();
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::delete_nodes_from_linked_list::{construct_list, Solution};

    #[test]
    fn case1() {
        let head = construct_list(vec![1, 2, 3, 4, 5]);
        let res = construct_list(vec![4, 5]);
        assert_eq!(Solution::modified_list(vec![1, 2, 3], head), res);
    }
}
