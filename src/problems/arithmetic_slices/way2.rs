use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(_nums: Vec<i32>) -> i32 {
        let mut _len = _nums.len();
        let mut result = 0;

        if _len < 3 {
            return result;
        }

        let mut _d = _nums[1] - _nums[0];
        let mut window_len = 1;

        for i in 1.._len {
            let _d_in_loop = _nums[i] - _nums[i - 1];

            if _d_in_loop != _d {
                _d = _d_in_loop;
                if window_len >= 3 {
                    let add = ((window_len - 2) * (window_len - 2 + 1)) / 2;
                    result += add;
                }
                window_len = 2;
            } else {
                window_len += 1;
            }
        }

        if window_len > 2 {
            let add = ((window_len - 2) * (window_len - 2 + 1)) / 2;
            result += add;
        }

        result
    }
}

pub fn solve(input_str: &str) {
    if input_str == "input-format" {
        println!("1 input -> Vec<i32>");
        println!("1 output -> i32");
        println!("example : {} => {:?}", "1,2,3,7,3,2,1,7,1,2,3,7,1,2,3", 4);
        return;
    }

    let nums_param = func_run::Format::to_vec_i32(0, input_str);
    let output = &Solution::number_of_arithmetic_slices(nums_param).to_string();

    func_run::solve_log("Repeated DNA Sequences", input_str, output);
}
