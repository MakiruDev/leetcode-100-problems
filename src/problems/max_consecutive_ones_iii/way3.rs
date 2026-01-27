use std::collections::VecDeque;

use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn longest_ones(_nums: Vec<i32>, _k: i32) -> i32 {
        let _len = _nums.len();
        let mut result = 0;
        let mut window_len = 0;
        let mut zero_idx = VecDeque::new();

        if _k >= _len as i32 {
            return _len as i32;
        }

        for right in 0.._len {
            if _nums[right] == 0 {
                if (zero_idx.len() == _k as usize && _k > 0) || _k == 0 {
                    if result < window_len {
                        result = window_len;
                    }

                    if _k == 0 {
                        window_len = -1;
                    } else {
                        window_len = (right - zero_idx[0] - 1) as i32;
                        zero_idx.pop_front();
                    }
                }
                zero_idx.push_back(right);
            }
            window_len += 1;
        }

        if result < window_len {
            result += window_len - result;
        }

        result
    }
}

pub fn solve(input_str: &str) {
    if input_str == "input-format" {
        println!("2 input -> 1,1,1,0,0,0,1,1,1,1,0 2");
        println!("1 output -> 6");
        println!("example : 1,1,1,0,0,0,1,1,1,1,0 2 => 6");
        return;
    }

    let param1 = func_run::Format::to_vec_i32(0, input_str);
    let param2 = func_run::Format::to_i32(1, input_str);

    let output = &Solution::longest_ones(param1, param2).to_string();

    func_run::solve_log("longest_ones", input_str, output);
}
