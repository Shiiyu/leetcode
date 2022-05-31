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
