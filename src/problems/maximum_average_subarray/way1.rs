use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn find_max_average(_nums: Vec<i32>, _k: i32) -> f64 {
        let num_len = _nums.len() as i32;
        let mut pre_result = 0;
        let mut find_max = 0;

        for i in 0.._k as usize {
            pre_result += _nums[i];
        }
        find_max += pre_result;

        for j in 0..(num_len - _k) as usize {
            let next = _nums[j + (_k as usize)];
            find_max += next - _nums[j];

            if pre_result < find_max {
                pre_result += find_max - pre_result;
            }
        }

        let result = pre_result as f64 / _k as f64;
        result
    }
}

pub fn solve(input_str: &str) {
    let nums_param = func_run::Format::to_vec_i32(0, input_str);
    let k_param = func_run::Format::to_i32(1, input_str);

    let output = &Solution::find_max_average(nums_param, k_param).to_string();

    func_run::solve_log("Maximum Average Subarray", input_str, output);
}
