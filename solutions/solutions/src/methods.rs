pub mod missing_two; // [面试题 17.19] 消失的两个数字
pub mod time_required_to_buy; // [2073] 买票需要的时间
pub mod maximize_win; // [2555] 两个线段获得的最多奖品
pub mod max_product; // 1339
pub mod check_if_prerequisite;
pub mod number_of_ways;
pub mod unhappy_friends;
pub mod get_folder_names;
pub mod task_scheduler_ii;
pub mod minimum_lines;
pub mod min_swaps;
pub mod can_choose;
pub mod ways_to_make_fair;
pub mod get_sum_absolute_differences;
pub mod display_table;
pub mod matrix_block_sum;
pub mod sample_stats;
pub mod atm;
pub struct Solution {}

// 二维数组转二维Vec
#[macro_export]
macro_rules! arr2_to_vec2 {
    // 匹配任意数量的逗号分隔的表达式
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // 对匹配到的每个表达式执行对应操作
            $(
                temp_vec.push($x.to_vec());
            )*
            temp_vec
        }
    };
}