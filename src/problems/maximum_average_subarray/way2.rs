use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn find_max_average(_nums: Vec<i32>, _k_param: i32) -> f64 {
        let _k = _k_param as usize;
        let num_len = _nums.len();
        let mut now_sum = 0;
        let mut max_sum;

        for i in 0.._k {
            now_sum += _nums[i];
        }
        max_sum = now_sum;

        for j in 0..(num_len - _k) {
            let next = _nums[j + _k];
            now_sum += next - _nums[j];

            if now_sum > max_sum {
                max_sum = now_sum;
            }
        }

        let result = max_sum as f64 / _k as f64;
        result
    }
}

pub fn solve(input_str: &str) {
    let nums_param = func_run::Format::to_vec_i32(0, input_str);
    let k_param = func_run::Format::to_i32(1, input_str);

    let output = &Solution::find_max_average(nums_param, k_param).to_string();

    func_run::solve_log("Maximum Average Subarray", input_str, output);
}
