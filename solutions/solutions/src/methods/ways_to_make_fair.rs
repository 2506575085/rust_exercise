use super::Solution;

impl Solution {
  pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let mut offset_cur = nums[1..nums.len()].iter().enumerate()
        .fold(0, |acc,(i,num)| if i % 2 == 0 { acc + num } else { acc - num });
    let mut res = if offset_cur == 0 {1} else {0};
    for i in 1..nums.len() {
        let last_num = nums[i-1];
        let num = nums[i];
        offset_cur = if i % 2 == 1 { offset_cur + last_num - num } else { offset_cur - last_num + num };
        if offset_cur == 0 {
            res += 1;
        }
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{:?}", Solution::ways_to_make_fair(vec![2,1,6,4]));
  }
}