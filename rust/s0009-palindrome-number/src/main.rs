impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    let y = x.to_string();

    y == y.chars().rev().collect::<String>()
  }
}
