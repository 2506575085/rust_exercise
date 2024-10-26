use super::Solution;

impl Solution {
  // preferences = [[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]]
  // 自身和本组成员的亲近度 小于自身和另一组某成员的亲近度 
  // （自身亲近度顺序表中 本组成员前的任一成员 在另一组）
  // 满足上述条件的成员中 存在和其同组成员亲近度小于 该成员和自身的亲近度 
  // （且满足上述条件的成员中 其同组成员在上述成员亲近度表中顺序 在自身后）
  // pairs = [[0, 1], [2, 3]]
  pub fn unhappy_friends(_n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    fn func(p1:usize,p2:usize,other_pairs: &Vec<&Vec<i32>>,preferences: &Vec<Vec<i32>>,res:&mut i32) {
        let p1_sort = &preferences[p1];
        let index = p1_sort.iter().position(|&x| x == p2 as i32).unwrap();
        let check_vec = p1_sort[0..index].to_vec();
        for check in &check_vec {
            let other_pair = other_pairs.iter().find(|&x| x.contains(&check)).unwrap();
            let other_p1 = *check as usize;
            let other_p2 = *other_pair.iter().find(|&x| *x != *check).unwrap() as usize;
            let other_p1_sort = &preferences[other_p1];
            let other_index = other_p1_sort.iter().position(|&x| x == other_p2 as i32).unwrap();
            let other_self_index = other_p1_sort.iter().position(|&x| x == p1 as i32).unwrap();
            if other_index > other_self_index {
                *res+=1;
                break;
            }
        }
    }
    let mut res = 0;
    for i in 0..pairs.len() {
        let pair = &pairs[i];
        let other_pairs = pairs.iter().filter(|&x| x != pair).collect::<Vec<&Vec<i32>>>();
        let p1 = pair[0] as usize;
        let p2 = pair[1] as usize;
        func(p1, p2, &other_pairs, &preferences, &mut res);
        func(p2, p1, &other_pairs, &preferences, &mut res);
    }
    res
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;
  use crate::arr2_to_vec2;
  #[test]
  fn it_works() {
    println!("{}", Solution::unhappy_friends(4, arr2_to_vec2![[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]], arr2_to_vec2![[1, 3], [0, 2]]));
  }
}