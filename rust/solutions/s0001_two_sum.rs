pub struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num_one) in nums.iter().enumerate() {
      for (j, num_two) in nums.iter().enumerate().skip(i + 1) {
        if num_one + num_two == target {
          return vec![i as i32, j as i32];
        }
      }
    }

    unreachable!();
  }
}

#[test]
fn test() {
  assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
