//! Leet Code - 002 - Add Two Numbers
//!
//! You are given two non-empty linked lists representing two non-negative
//! integers. The digits are stored in reverse order, and each of their nodes
//! contains a single digit. Add the two numbers and return the sum as a linked
//! list.
//!
//! You may assume the two numbers do not contain any leading zero, except the
//! number 0 itself.

use std::time::Instant;

pub(crate) fn two_main() {
    println!("\nLeet Code - 002 - Add Two Numbers");

    let l1 = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));

    let l2 = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 5, next: None })),
        })),
    }));

    let now = Instant::now();

    let mut cur_a = l1;
    let mut cur_b = l2;
    let mut soln = Some(Box::new(ListNode::new(0)));
    let mut soln_next = &mut soln;

    while cur_a.is_some() || cur_b.is_some() {
        let (val_a, next_a) = match cur_a {
            Some(node) => (node.val, node.next),
            None => (0, None),
        };
        cur_a = next_a;

        let (val_b, next_b) = match cur_b {
            Some(node) => (node.val, node.next),
            None => (0, None),
        };
        cur_b = next_b;

        match soln_next {
            Some(node) => {
                let cur_sum = node.val + val_a + val_b;
                node.val = cur_sum % 10;
                if cur_sum > 9 || cur_a.is_some() || cur_b.is_some() {
                    node.next
                        .get_or_insert(Box::new(ListNode::new(cur_sum / 10)));
                }
                soln_next = &mut node.next;
            }
            None => unreachable!(),
        }
    }

    println!("Output: {soln:#?}");

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

/// Provided definition for singly-linked list.
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
