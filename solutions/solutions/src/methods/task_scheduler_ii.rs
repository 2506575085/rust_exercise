use std::collections::HashMap;

use super::Solution;

impl Solution {
  pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
    let mut next_day_map: HashMap<i32, i64>  = HashMap::from([(tasks[0],space as i64 + 1)]);
    let mut now_day: i64 = 1;
    for index in 1..tasks.len() {
        let task_type = tasks[index];
        if let Some(&next_day) = next_day_map.get(&task_type) {
            let offset = next_day - now_day;
            if offset > 0 {
                now_day += offset + 1;
            } else {
                now_day += 1;
            }
        } else {
            now_day += 1;
        }
        next_day_map.insert(task_type, now_day + space as i64);
    }
    now_day
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{}", Solution::task_scheduler_ii(vec![5,8,8,5], 2))
  }
}