use super::Solution;

impl Solution {
  pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
    if stock_prices.len() == 1 {
        return 0;
    }
    let mut sorted_points = stock_prices;
    sorted_points.sort_by_key(|coordinate| coordinate[0]);
    sorted_points.iter().enumerate()
        .fold(1, |acc,( index, point )| {
            if index == sorted_points.len() - 1 || index == 0 {
                return acc;
            }
            let next_point = &sorted_points[index+1];
            let last_point = &sorted_points[index-1];
            let left = (next_point[1] - point[1]) as i128 * (point[0] - last_point[0]) as i128;
            let right = (next_point[0] - point[0]) as i128 * (point[1] - last_point[1]) as i128;
            if left != right {
                return acc+1;
            }
            acc
        })
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  #[test]
  fn it_works() {
    println!("{}", Solution::minimum_lines(crate::arr2_to_vec2![[1,1],[500000000,499999999],[1000000000,999999998]]));
  }
}