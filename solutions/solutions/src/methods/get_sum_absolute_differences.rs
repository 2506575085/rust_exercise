use super::Solution;

impl Solution {
  pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let mut left_sum = 0;
    let mut right_sum = nums.iter().sum::<i32>();
    let mut res: Vec<i32> = vec![];
    for i in 0..nums.len() {
        let num = nums[i];
        let num_res = num*i as i32 - left_sum + right_sum - (nums.len()-i) as i32 * num;
        res.push(num_res);
        left_sum += num;
        right_sum -= num;
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{:?}", Solution::get_sum_absolute_differences(vec![2,3,5]));
  }
}