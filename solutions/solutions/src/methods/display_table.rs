use super::Solution;

impl Solution {
  pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut res = vec![vec![]];
    for order in &orders{
        let name_row: &mut Vec<String> = &mut res[0];
        let name = order[2].clone();
        if name_row.into_iter()
                .position(|x| x.to_string() == name)
                .is_none() {
            name_row.push(name);
        }
    }
    res[0].sort();
    res[0].insert(0, String::from("Table"));
    for order in &orders {
        let name = order[2].clone();
        let table_number = order[1].clone();
        let name_row = res[0].clone();
        let table_row = res.iter_mut().find(|row| {
            if row[0] == table_number.to_string() {
                return true;
            } else {
                return false;
            }
        });
        match table_row {
            Some(row) => {
                let name_index = 
                    name_row.iter().position(|name_| name_.to_string() == name);
                if let Some(index) = name_index {
                    let row_index: usize = row[index].parse().unwrap();
                    row[index] = (1 + row_index).to_string();
                } else {
                    res[0].push(name.to_string());
                    for row_i in 1..res.len() {
                        if res[row_i][0] == table_number.to_string() {
                            res[row_i].push(String::from("1"));
                        } else {
                            res[row_i].push(String::from("0"));
                        }
                    }
                }
            },
            None => {
                let mut new_row = vec![String::from("0"); res[0].len()];
                new_row[0] = table_number.clone();
                let name_index = 
                    name_row.iter().position(|name_| name_.to_string() == name);
                if let Some(index) = name_index {
                    new_row[index] = String::from("1");
                } else {
                    res[0].push(name.to_string());
                    for row_i in 1..res.len() {
                        res[row_i].push(String::from("0"));
                    }
                    new_row.push(String::from("1"));
                }
                res.push(new_row);
            }
        }
    }
    res.sort_by(|a, b| {
        let a_name = a[0].clone();
        let b_name = b[0].clone();
        let a_name = if a_name == String::from("Table") {
            -1
        } else {
            a_name.parse().unwrap()
        };
        let b_name = if b_name == String::from("Table") {
            -1
        } else {
            b_name.parse().unwrap()
        };
        a_name.cmp(&b_name)
    });
    res
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    let arr = [
        ["David","3","Ceviche"],
        ["Corina","10","Beef Burrito"],
        ["David","3","Fried Chicken"],
        ["Carla","5","Water"],
        ["Carla","5","Ceviche"],
        ["Rous","3","Ceviche"]
    ];
    let orders = arr.iter().map(|x| x.iter().map(|&y| y.to_string()).collect()).collect();
    println!("{:?}", Solution::display_table(orders));
  }
}