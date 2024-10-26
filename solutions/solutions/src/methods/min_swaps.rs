use super::Solution;

impl Solution {
  pub fn min_swaps(s: String) -> i32 {
    let mut c = 0;
    let mut ans = 0;
    for char in s.chars() {
        if char == '[' {
            c += 1;
        } else if c > 0 {
            c -= 1;
        } else {
            c += 1;
            ans += 1;
        }
    }
    ans
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{}", Solution::min_swaps(String::from("]]][[[")));
  }
}