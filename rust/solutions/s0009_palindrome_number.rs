pub struct Solution;

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    let y = x.to_string();

    y == y.chars().rev().collect::<String>()
  }
}

#[test]
fn test() {
  assert_eq!(Solution::is_palindrome(121), true);
  assert_eq!(Solution::is_palindrome(-121), false);
  assert_eq!(Solution::is_palindrome(10), false);
}
