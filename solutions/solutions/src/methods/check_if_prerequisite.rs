use std::collections::HashMap;

use super::Solution;
impl Solution {
  pub fn check(map: &HashMap<i32, Vec<i32>>, pre_type: i32, next_course: i32, res_map:&mut HashMap<i32, HashMap<i32,bool>>) -> bool {
      if let Some(cache) = res_map.get(&pre_type) {
          let res = cache.get(&next_course);
          if let Some(res) = res {
              return *res;
          }
      }
      if map.contains_key(&pre_type) {
          let check_vec = map.get(&pre_type).unwrap();
          if check_vec.contains(&next_course) {
              if let Some(cache) = res_map.get_mut(&pre_type) {
                  cache.insert(next_course, true);
              } else {
                  res_map.insert(pre_type, HashMap::from([(next_course, true)]));
              }
              return true;
          } else {
              for pre in check_vec {
                  let flag = Self::check(map, *pre, next_course, res_map);
                  if let Some(cache) = res_map.get_mut(pre) {
                      cache.insert(next_course, flag);
                  } else {
                      res_map.insert(*pre, HashMap::from([(next_course, flag)]));
                  }
                  if flag {
                      return true;
                  }
              }
              return false;
          }
      } else {
          if let Some(cache) = res_map.get_mut(&pre_type) {
              cache.insert(next_course, false);
          } else {
              res_map.insert(pre_type, HashMap::from([(next_course, false)]));
          }
          return false;
      }
  }
  
  pub fn check_if_prerequisite(_num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
      let mut pre_map: HashMap<i32, Vec<i32>> = HashMap::new();
      for pre in prerequisites {
          let pre_type = pre[0];
          let next_course = pre[1];
          if let Some(next) = pre_map.get_mut(&pre_type) {
              if !next.contains(&next_course) { next.push(next_course); }
          } else {
              pre_map.insert(pre_type, vec![next_course]);
          }
      }
  
      let mut res_map: HashMap<i32, HashMap<i32,bool>> = HashMap::new();
      queries.iter().map(|query| {
          let pre_type = query[0];
          let next_course = query[1];
          Self::check(&pre_map, pre_type, next_course, &mut res_map)
      }).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  use crate::arr2_to_vec2;
  #[test]
  fn it_works() {
    let prerequisites = arr2_to_vec2![[1,0]];
    let queries = arr2_to_vec2![[0,1],[1,0]];
    println!("{:?}", Solution::check_if_prerequisite(33, prerequisites, queries));
  }
}