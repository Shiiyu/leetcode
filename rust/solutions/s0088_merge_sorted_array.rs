pub struct Solution;

impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in 0..(n as usize) {
      nums1[m as usize + i] = nums2[i];
    }

    nums1.sort_unstable();
  }
}

#[test]
fn test() {
  let mut vec1 = vec![1, 2, 3, 0, 0, 0];
  let mut vec2 = vec![2, 5, 6];
  Solution::merge(&mut vec1, 3, &mut vec2, 3);
  assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);

  let mut vec1 = vec![1];
  let mut vec2 = Vec::new();
  Solution::merge(&mut vec1, 1, &mut vec2, 0);
  assert_eq!(vec1, vec![1]);

  let mut vec1 = vec![0];
  let mut vec2 = vec![1];
  Solution::merge(&mut vec1, 0, &mut vec2, 1);
  assert_eq!(vec1, vec![1]);
}
