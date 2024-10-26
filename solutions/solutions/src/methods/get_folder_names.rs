use std::collections::HashMap;

use super::Solution;

impl Solution {
  pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut name_map: HashMap<String, u32> = HashMap::new();
    fn func(name: &str, name_map: &mut HashMap<String, u32>) -> String {
        if let Some(num) = name_map.get_mut(name) {
            let new_name = format!("{name}({num})");
            *num += 1;
            if let Some(_) = name_map.get_mut(&new_name) {
                return func(name, name_map);
            } else {
                name_map.insert(new_name.clone(), 1);
                return new_name;
            }
        } else {
            name_map.insert(name.to_string(), 1);
            return name.to_string();
        }
    }
    names.into_iter().map(|s|func(&s, &mut name_map)).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{:?}", Solution::get_folder_names(vec!["kaido","kaido(1)","kaido","kaido(1)"].into_iter().map(|s|s.to_string()).collect()));
  }
}