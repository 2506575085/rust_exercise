use super::Solution;

impl Solution {
  // 从第一个与第0个字符不同的字符开始 到最后一个与末尾字符不同的字符结束 （前方与自身不同的个数*后方与自身不同的个数）的和
  // s.chars().nth(i).unwrap(); //这玩意巨慢、慎用
  pub fn number_of_ways(s: String) -> i64 {
    let (one_count, zero_count) = s.chars().fold((0,0), |acc,x|{
        if x == '1' {
            (acc.0+1,acc.1)
        } else {
            (acc.0,acc.1+1)
        }
    });
    let mut left_count = (0,0);
    s.chars().fold(0, |acc,x| {
        if x == '1' {
            left_count.1 += 1;
            acc+(left_count.0*(zero_count - left_count.0)) as i64
        } else {
            left_count.0 += 1;
            acc+(left_count.1*(one_count - left_count.1)) as i64
        }
    })
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{}", Solution::number_of_ways(String::from("01010")));
  }
}