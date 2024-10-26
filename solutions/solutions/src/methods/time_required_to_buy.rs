use super::Solution;
impl Solution {
  // [2073] 买票需要的时间
  pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
      let need_tickets = tickets[k as usize];
      let arr: Vec<i32> = tickets
          .iter()
          .map(|&value| value - need_tickets)
          .collect();
      
      let mut offset = 0;
      for index in k as usize + 1..arr.len() {
          if arr[index] >= 0 {
              offset += 1;
          }
      }
      let over = arr.iter().fold(0,|acc,&v| if v < 0 { acc + v } else { acc });
      println!("arr:{:?}\nlen:{}\nneed:{}\noffset:{}\nover:{}",arr,arr.len(),need_tickets,offset,over);
      need_tickets * tickets.len() as i32 + over - offset
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
    fn it_works() {
      println!("{}", Solution::time_required_to_buy(vec![15,66,3,47,71,27,54,43,97,34,94,33,54,26,15,52,20,71,88,42,50,6,66,88,36,99,27,82,7,72],18));
    }
}