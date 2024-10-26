use super::Solution;

impl Solution {
  pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
    let mut minimum = -1.0;
    let mut maximum = 0.0;
    let mut mean = 0.0;
    let mut total = 0.0;
    let mut mode_ = (0.0,0);
    for i in 0..count.len() {
        let count_num = count[i] as f64;
        let i = i as f64;
        if count_num != 0.0 {
            if minimum == -1.0 {
                minimum = i;
            }
            maximum = i;
            mean += i * count_num;
            total += count_num;
            if count_num > mode_.0 as f64 {
                mode_ = (count_num, i as usize);
            }
        }
    }
    let median = if total as i32 % 2 == 0 {
        let mut left_count = 0;
        let mut num = 0.0;
        let total = total as i32;
        for i in 0..count.len() {
            let count_num = count[i];
            if (left_count + count_num) >= (total / 2) {
                if (left_count + count_num) == (total / 2) {
                    let first_i = i as f64;
                    for next_i in i+1..count.len() {
                        if count[next_i] != 0 {
                            num = (first_i + next_i as f64) / 2.0;
                            break;
                        }
                    }
                    break;
                } else {
                    num = i as f64;
                    break;
                }
            }
            left_count += count_num;
        }
        num
    } else {
        let mut left_count = 0;
        let mut num = 0.0;
        for i in 0..count.len() {
            let count_num = count[i];
            if (left_count + count_num) >= (total as i32 / 2 + 1) {
                num = i as f64;
                break;
            }
            left_count += count_num;
        }
        num
    };
    vec![minimum, maximum, mean / total, median, mode_.1 as f64]
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{:#?}", Solution::sample_stats(Vec::from([0,4,3,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])));
  }
}