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
    let now = Instant::now();

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
