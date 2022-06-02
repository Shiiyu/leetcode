pub struct Solution;

impl Solution {
  pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
  }
}

#[test]
fn test() {
  assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
  assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
  assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_string()), 6)
}
