// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>, // Struct std::boxed::Box => A pointer type that uniquely owns a heap allocation of type T.
}

impl ListNode {
    #[inline] // How to use inline to speed up? => https://nnethercote.github.io/perf-book/inlining.html ???
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_number(val: i32) -> Self {
        if val < 10 {
            ListNode::new(val)
        } else {
            ListNode {
                val: val % 10,
                next: Some(Box::new(ListNode::from_number(val / 10))),
            }
        }
    }

    fn sum(&self) -> i32 {
        let mut total = self.val;
        let mut node = &self.next;
        let mut counter = 1;
        loop {
            match node {
                Some(some_node) => {
                    total += some_node.val * 10_i32.pow(counter); // raise a number to a power => https://stackoverflow.com/a/51208704
                    counter += 1;
                    node = &some_node.next;
                }
                None => {
                    break;
                }
            }
        }
        total
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(node1), Some(node2)) => {
                Some(Box::new(ListNode::from_number(node1.sum() + node2.sum())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::add_two_numbers::{ListNode, Solution};

    #[test]
    fn test_from_number() {
        let res = ListNode::from_number(321);
        assert_eq!(res.sum(), 321)
    }
    
    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from_number(342))),
                Some(Box::new(ListNode::from_number(465)))
            )
            .unwrap()
            .sum(),
            807
        );
    }
}
