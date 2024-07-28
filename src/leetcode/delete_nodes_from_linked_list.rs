use std::{borrow::BorrowMut, collections::HashSet};

use super::lexicographically_smallest_string;

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
        ListNode { next: None, val }
    }
}

fn construct_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
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
        let nums_to_remove: HashSet<i32> = nums.into_iter().collect();
        let mut new_head = &head;
        while !new_head.is_none() {
            let _node = new_head.as_ref();
            if nums_to_remove.contains(&_node?.val) {
                new_head = &_node?.next;
            } else {
                break;
            }
        }
        let mut last = &mut new_head.clone().unwrap();
        let mut node = & new_head.clone().unwrap().next;
        
        loop {
            match node {
                Some(_node) => {
                    if nums_to_remove.contains(&_node.val) {
                        (*last).next = _node.next.clone();
                    }
                    last = _node;
                    node = & _node.next;
                }
                None => {
                    break;
                }
            }
        }
        new_head.clone()
        
        // match head {
        //     Some(_head) => {
        //         let mut node = &_head;
        //         while !&node.next.is_none() && !nums_to_remove.contains(&node.val) {
        //             node = &(&node.next.unwrap());
        //         }
        //         match head {
        //             Some(_head) => {
        //                 let mut node = _head;
        //                 while nums_to_remove.contains(&node.val) && !node.next.is_none() {
        //                     node = node.next.unwrap();
        //                 }
        //                 let res = node.clone();

        //                 let mut last = node.clone();
        //                 while !node.next.is_none() {
        //                     if nums_to_remove.contains(&node.val) {
        //                         last.next = node.next.clone()
        //                     }
        //                     last = node.clone();
        //                     node = node.next.unwrap()
        //                 }
        //                 println!("{:?}", &res);
        //                 println!("{:?}", &last);
        //                 println!("{:?}", &node);
        //                 Some(res)
        //             }
        //             None => head,
        //         }
        //     }
        //     None => None,
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
