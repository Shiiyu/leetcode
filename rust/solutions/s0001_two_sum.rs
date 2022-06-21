use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::with_capacity(nums.len());

    for (i, n) in nums.iter().enumerate() {
      match hm.get(&(target - n)) {
        None => {
          hm.insert(n, i);
        },
        Some(su) => return vec![*su as i32, i as i32]
      }
    }
    unreachable!()
  }
}

#[test]
fn test() {
  assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
