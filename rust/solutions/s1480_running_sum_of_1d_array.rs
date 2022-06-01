pub struct Solution;

impl Solution {
  pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut running_sum = Vec::new();

    for i in (1..=nums.len()).rev() {
      running_sum.push(nums.iter().take(i).sum());
    }

    running_sum.reverse();
    running_sum
  }
}

#[test]
fn test() {
  assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
  assert_eq!(Solution::running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
  assert_eq!(Solution::running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17])
}
