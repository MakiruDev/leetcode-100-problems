use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn longest_ones(_nums: Vec<i32>, _k: i32) -> i32 {
        let _len = _nums.len();
        let mut result = 0;
        let mut count_k = _k;
        let mut window_len = 0;
        let mut idx_zero = vec![];

        for i in 0.._len {
            if _nums[i] == 0 {
                if count_k <= 0 && _k > 0 {
                    if result < window_len {
                        result += window_len - result;
                    }
                    count_k += 1;
                    window_len = (i - idx_zero[0] - 1) as i32;
                    idx_zero.remove(0);
                } else if _k == 0 {
                    if result < window_len {
                        result += window_len - result;
                    }
                    window_len = -1;
                }
                idx_zero.push(i);
                count_k -= 1;
            }
            window_len += 1;
            println!("i = {:?}, window_len = {:?}", i, window_len);
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
