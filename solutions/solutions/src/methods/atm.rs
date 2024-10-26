#[derive(Debug)]
pub struct ATM {
  count_list: [u32; 5],
  size_map: [u32;5],
}
impl ATM {
    pub fn new() -> Self {
        ATM {
            count_list: [0; 5],
            size_map: [20,50,100,200,500],
        }
    }
    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, &count) in banknotes_count.iter().enumerate() {
            self.count_list[i] += count as u32;
        }
    }
    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut amount = amount;
        let mut res = vec![0;5];
        let mut next_countlist = self.count_list;
        for i in (0..5).rev() {
            if amount == 0 {
                self.count_list = next_countlist;
                return res;
            }
            if amount >= self.size_map[i] as i32 
                && self.count_list[i] > 0 {
                let need_count = amount / self.size_map[i] as i32;
                let has_count = self.count_list[i] as i32;
                let min_count = std::cmp::min(need_count, has_count);
                amount -= self.size_map[i] as i32 * min_count;
                res[i] = min_count;
                next_countlist[i] -= min_count as u32;
            }
        }
        if amount == 0 {
            self.count_list = next_countlist;
            return res;
        }
        vec![-1]
    }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let mut atm = super::ATM::new();
    println!("{:?}", atm.deposit(vec![0,0,1,2,1]));
    println!("{:?}", atm.withdraw(600));
    println!("{:?}", atm.deposit(vec![0,1,0,1,1]));
    println!("{:?}", atm.withdraw(600));
    println!("{:?}", atm.withdraw(570));
  }
}