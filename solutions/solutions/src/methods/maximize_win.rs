use std::collections::HashMap;

use super::Solution;

impl Solution {
  // [2555] 两个线段获得的最多奖品
  // [1,1,2,2,3,3,5,7,7,7,7,8,9,9,9,9,9] 1
  // { 1: (self_count:2,count:4,other_max_count:6,next:2,pre:null),2:(2,4,null,3,1), 3: (2,2,null,5,2),
  //   5:(1,1,null,7,3), 7:(4,5,null,8,5), 8:(1,6,null,9,7), 9:(5,null,null,null,8) } left_max:0,right_max:6
  // { 2:(2,4,6,3,1), 3, 5, 7, 8, 9 } left_max: (pre.pre..到小于position-k以前的count最大值，若pre不足则为全部pre的self_count和) 2,
  //                                  right_max: (next.next..到大于position+k以后的count最大值,其他则为0) 6
  // { 3:(2,2,6,5,2),... } left_max: 4, right_max: 6
  // { 5:(1,1,6,7,3),... } left_max: 4, right_max: 6
  // { 7:(4,5,5,8,5),... } left_max: 4, right_max: 5
  // { 8:(1,6,4,9,7),... } left_max: 4, right_max: 0
  // { 9:(5,5,5,null,8) } left_max: 5, right_max: 0
  // { 1: (2,4,6,2,null), 2:(2,4,6,3,1), 3:(2,2,6,5,2), 5:(1,1,6,7,3) 7:(4,5,5,8,5) 8:(1,6,4,9,7), 9:(5,5,5,null,8) }

  // [1,1,2,2,3,3,5] 2 更新count: 初始化为self_count,将自身position-k到自身position范围内的所有pre的count+=self_count
  // {1:(2,2+2+2=6,1,2,null),2:(2,2+2=4,null,3,1),3:(2,2+1=3,null,5,2),5:(1,1,null,null,3)} left_max: 0, right_max: 1
  // {2:(2,4,null,3,1)} left_max: 2, right_max: 0

  // [1,1,2,2,3,3,5,7,7,7,7,7,8,8,11,11,11] 2
  // {
  //  1: (2, 6, 7, Some(2), None, None, Some(5)), l:0 r:7
  //  2: (2, 4, 7, Some(3), Some(1), None, Some(5)), l:2 r:7
  //  3: (2, 3, 7, Some(5), Some(2), None, Some(7)), l:4 r:7
  //  5: (1, 6, 6, Some(7), Some(3), Some(2), Some(8)), l:6 r:5
  //  7: (5, 7, 6, Some(8), Some(5), Some(3), Some(11)), l:6 r:3
  //  8: (2, 5, 6, Some(11), Some(7), Some(5), Some(11)), l:6 r:3
  //  11: (3, 3, 7, None, Some(8), Some(8), None), l:7 r:0
  // } 左边的最大值取左侧最靠近的不相交区间和与历史左侧最大值的最大值、右侧最大值反向遍历一次取两者最大覆盖到第三位、同时更新最终最大值
  // 13
  pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
    // 0 self_count, 1 count, 2 other_max_count, 3 next, 4 pre, 5 k_pre, 6 k_next
    let start_position = *prize_positions.first().unwrap();
    let final_position = *prize_positions.last().unwrap();
    if k >= final_position - start_position {
        return prize_positions.len() as i32;
    }
    let mut step_map = HashMap::<i32,(i32,i32,i32,Option<i32>,Option<i32>,Option<i32>,Option<i32>)>::new();
    let mut last_position = start_position;
    let mut last_index = 0;
    for (index, &position) in prize_positions.iter().enumerate() {
        let index = index as i32;
        let self_count = index - last_index;
        if position != last_position {
            if let Some(step) = step_map.get_mut(&last_position) {
                step.0 += self_count;
                step.1 += self_count;
                step.3 = Some(position);
                let min_pre = last_position - k;
                let step_pre = step.4.clone();
                Self::update_k_step(step_pre, &mut step_map, min_pre, self_count);
            } else {
                step_map.insert(last_position, (self_count, self_count, 0, Some(position), None, None, None));
            }
            step_map.insert(position, (0, 0, 0, None, Some(last_position), None, None));
            last_position = position;
            last_index = index;
        }
    }
    let self_count = prize_positions.len() as i32 - 1 - last_index;
    let self_count = if final_position == last_position { self_count + 1 } else { 1 };
    let last_position = if final_position == last_position { last_position } else { final_position };
    let step = step_map.get_mut(&last_position).unwrap();
    step.0 = self_count;
    step.1 = self_count;
    let min_pre = last_position - k;
    let step_pre = step.4.clone();
    Self::update_k_step(step_pre, &mut step_map, min_pre, self_count);

    let mut loop_position = start_position;
    let mut left_count = 0;
    let mut left_max_record = 0;
    while let Some(current_step) = step_map.get(&loop_position) {
        let next_position = current_step.3.clone();
        let current_count = current_step.0.clone();
        
        if let Some(pre_k) = current_step.5 {
            let current_k_next = current_step.6.clone();
            let pre_position = step_map.get(&pre_k).unwrap().4;
            if let Some(pre_pos) = pre_position {
                let mut cur_pos = pre_pos;
                while let Some(pre_step) = step_map.get(&cur_pos) {
                    let pre = pre_step.4.clone();
                    left_max_record = left_max_record.max(pre_step.1).max(left_count);
                    step_map.get_mut(&loop_position).unwrap().2 = left_max_record;

                    //   let cur_k = step_map.get(&cur_pos).unwrap().6.unwrap_or(-1);
                    //   let cur_k_cur = current_k_next.unwrap_or(-2);
                    //   if cur_k != cur_k_cur {
                    //       break;
                    //   }
                    match (current_k_next, step_map.get(&cur_pos).unwrap().6) {
                        (Some(a), Some(b)) if a == b => {}
                        _ => { break; }
                    }
                    
                    if let Some(pre) = pre {
                        cur_pos = pre;
                    }
                }
            }
        } else {
            left_max_record = left_max_record.max(left_count);
            step_map.get_mut(&loop_position).unwrap().2 = left_max_record;
        }
        if loop_position - k <= start_position {
            left_count += current_count;
        }
        
        if let Some(next_loop_position) = next_position {
            loop_position = next_loop_position;
        } else {
            break;
        };
    }

    let mut loop_position = final_position;
    let mut right_max = 0;
    let mut res = 0;
    while let Some(&(_, count, other_max_count, _, pre_position, _, k_next)) = step_map.get(&loop_position) {
        if let Some(k_next) = k_next {
            right_max = right_max.max(step_map.get(&k_next).unwrap().1);
            res = res.max(count + right_max.max(other_max_count));
            let mut next_loop_position = step_map.get(&k_next).unwrap().3;
            while let Some(next) = next_loop_position {
                match (step_map.get(&next).unwrap().5, step_map.get(&k_next).unwrap().5) {
                    (Some(a), Some(b)) if a == b => {}
                    _ => { break; }
                }
                //   let next_k = step_map.get(&next).unwrap().5.unwrap_or(-1);
                //   let next_k_next = step_map.get(&k_next).unwrap().5.unwrap_or(-2);
                //   if next_k != next_k_next {
                //       break;
                //   }
                right_max = right_max.max(step_map.get(&next).unwrap().1);
                res = res.max(count + right_max.max(other_max_count));
                next_loop_position = step_map.get(&next).unwrap().3;
            }
        } else {
            res = res.max(count + other_max_count);
        }

        if let Some(next_loop_position) = pre_position {
            loop_position = next_loop_position;
        } else {
            break;
        }
    }

    // println!("{:?}", step_map);
    res
  }
  fn update_k_step(step_pre_clone: Option<i32>, step_map: &mut HashMap<i32, (i32, i32, i32, Option<i32>, Option<i32>, Option<i32>,Option<i32>)>, min_pre: i32, self_count: i32) {
      let mut step_pre = step_pre_clone;
      while let Some(pre_position) = step_pre {
          let step = step_map.get_mut(&pre_position).unwrap();
          if pre_position >= min_pre {
              step.1 += self_count;
          }
          if pre_position < min_pre {
              if let Some(set_position) = step_pre_clone {
                  step_map.get_mut(&step_map.get(&set_position).unwrap().3.unwrap()).unwrap().5 = step_pre;
                  if let None = step_map.get(&pre_position).unwrap().6 {
                      step_map.get_mut(&pre_position).unwrap().6 = step_map.get(&set_position).unwrap().3;
                  }
                  
                  let mut pre_pos = step_map.get(&pre_position).unwrap().4;
                  while let Some(pre) = pre_pos {
                      if let Some(_) = step_map.get(&pre).unwrap().6 {
                          break;
                      }
                      step_map.get_mut(&pre).unwrap().6 = step_map.get(&set_position).unwrap().3;
                      pre_pos = step_map.get(&pre).unwrap().4;
                  }

              }
              break;
          }
          step_pre = step.4;
      }
  }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
      println!("{}", Solution::maximize_win(vec![1,1,2,2,3,3,5], 2));
      println!("{}", Solution::maximize_win(vec![1,1,2,2,3,3,5,7,7,7,7,7,8,8,11,11,11], 2));
    }
}