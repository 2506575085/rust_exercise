use super::Solution;

impl Solution {
  pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let row_len = mat.len();
    let col_len = mat[0].len();
    let mut res = vec![vec![0; col_len]; row_len];
    for row_index in 0..row_len {
        for col_index in 0..col_len {
            for next_row_index in 
                std::cmp::max(0, row_index as i32 - k as i32) as usize
                ..=std::cmp::min(row_len-1, row_index + k as usize) {
                for next_col_index in 
                    std::cmp::max(0, col_index as i32 - k as i32) as usize
                    ..=std::cmp::min(col_len-1, col_index + k as usize) {
                    res[row_index][col_index] += mat[next_row_index][next_col_index];
                }
            }
        }
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use crate::arr2_to_vec2;

  use super::Solution;
  #[test]
  fn it_works() {
    println!("{:?}", Solution::matrix_block_sum(arr2_to_vec2![[1,2,3],[4,5,6],[7,8,9]], 1));
  }
}