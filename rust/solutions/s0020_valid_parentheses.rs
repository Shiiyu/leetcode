pub struct Solution;

impl Solution {
  pub fn equals(a: char, b: char) -> bool {
    match a {
      '(' => b == ')',
      '[' => b == ']',
      '{' => b == '}',
      _ => unreachable!()
    }
  }

  pub fn is_valid(s: String) -> bool {
    let mut openings = Vec::new();

    for c in s.chars() {
      if matches!(c, '(' | '[' | '{') {
        openings.push(c);
      } else if !openings.is_empty() && Solution::equals(openings[openings.len() - 1], c) {
        openings.pop();
      } else if matches!(c, ')' | ']' | '}') {
        return false;
      }
    }

    openings.is_empty()
  }
}

#[test]
fn test() {
  assert_eq!(Solution::is_valid("()".to_string()), true);
  assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
  assert_eq!(Solution::is_valid("(]".to_string()), false);
}
