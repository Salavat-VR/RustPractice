//Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.
//You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
//You can return the answer in any order.

use std::collections::HashMap;

fn main() {}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        // if we have the situation matches the demand

        let search = target - num;

        if let Some(&j) = hm.get(&search) {
            return vec![j, i as i32];
        } else {
            hm.insert(num, i as i32);
        }
    }
    // otherwise
    vec![]
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(two_sum(nums, target), vec![0, 1]);
}
