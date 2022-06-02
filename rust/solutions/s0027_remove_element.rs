pub struct Solution;

impl Solution {
  pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&n| n != val);
    nums.len() as i32
  }
}

#[test]
fn test() {
  assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
  assert_eq!(Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
}
