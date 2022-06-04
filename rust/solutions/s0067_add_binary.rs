pub struct Solution;

impl Solution {
  pub fn add_binary(a: String, b: String) -> String {
    format!("{:b}", u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap())
  }
}

#[test]
fn test() {
  assert_eq!(Solution::add_binary("11".to_string(), "1".to_string()), "100".to_string());
  assert_eq!(Solution::add_binary("1010".to_string(), "1011".to_string()), "10101".to_string());
}
