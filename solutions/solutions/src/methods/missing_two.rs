use std::cell::RefCell;

use super::Solution;
// [面试题 17.19] 消失的两个数字
impl Solution {
  pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
    let first = nums[0];
    let nums = RefCell::new(vec![nums,vec![first,first]].concat());
    let mut nums = nums.borrow_mut();
    for i in 0..nums.len() {
      let index = nums[i].abs() as usize - 1;
      if nums[index] > 0 { nums[index] *= -1; }
    }
    nums
      .iter()
      .enumerate()
      .map(|(i, v)| if *v > 0 { i as i32 + 1 } else { -1 })
      .filter(|&v| v != -1)
      .collect()
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
    fn it_works() {
      println!("{:?}", Solution::missing_two(vec![2,3]));
      assert!(Solution::missing_two(vec![2,3]) == vec![1,4]);
    }
}