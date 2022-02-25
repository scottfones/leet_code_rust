//! Leet Code - 001 - Two Sum
//!
//! Given an array of integers `nums` and an integer `target`, return indices of the
//! two numbers such that they add up to `target`. You may assume that each input
//! would have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.

use std::collections::HashMap;
use std::time::Instant;

pub(crate) fn one_main() {
    println!("\nLeet Code - 001 - Two Sum");
    let now = Instant::now();

    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let sol = two_sum(nums, target);
    println!("001 - Two Sum: {sol:?}");

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diff_map: HashMap<i32, i32> = HashMap::new();

    for (i, val) in nums.into_iter().enumerate() {
        match diff_map.get(&val) {
            Some(&idx) => return vec![idx, i as i32],
            None => {
                diff_map.insert(target - val, i as i32);
            }
        }
    }
    Vec::new()
}
