use super::Solution;

impl Solution {
  pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    let mut last_index = 0;
    'a: for to_arr in groups {
        let to_arr_len = to_arr.len();
        let mut current_len = 0;
        let mut i = last_index;
        let mut next_i = 0;
        while i < nums.len() {
            let num = nums[i];
            if num == to_arr[current_len] {
                current_len += 1;
            } else {
                current_len = 0;
            }
            if num == to_arr[0] && next_i == 0 {
                next_i = i;
            }
            if current_len == to_arr_len {
                last_index = i + 1;
                continue 'a;
            }
            i += 1;
            if (i == nums.len()) && (next_i != 0) {
                i = next_i + 1;
                next_i = 0;
                current_len = 1;
            }
        }
        return false;
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{}", Solution::can_choose(crate::arr2_to_vec2![[10,-2],[1,2,3,4]],vec![1,2,3,4,10,-2]));
  }
}