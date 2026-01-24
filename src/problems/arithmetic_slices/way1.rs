use crate::problems::func_run;
// use std::collections::{HashMap, hash_map::Entry};

pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(_nums: Vec<i32>) -> i32 {
        let mut window_len = 1;
        // // let mut window_len_back = 1;
        // let mut window_num = 0;
        // let mut we_in = "front";
        // let mut window_count = 0;
        let mut loop_one = true;
        let mut _d = 0;
        let mut _len = _nums.len();
        let mut result = 0;
        // let mut _map: HashMap<i32, i32> = HashMap::new();
        // result += 1;
        let mut new = false;

        if _len < 3 {
            return result;
        }

        for i in 0.._len {
            // let now = _nums[i];

            if loop_one {
                print!("i = {:?}, window_len = {:?}\n", i, window_len);
                _d = _nums[1] - _nums[0];
                // _map.insert(window_count, _d);
                loop_one = false;
                continue;
            }
            let _d_in_loop = _nums[i] - _nums[i - 1];
            let window_check = _d_in_loop != _d;

            if new {
                if !window_check {
                    window_len = 3;
                    new = false;
                    print!("i = {:?}, window_len = {:?}, new = true\n", i, window_len);
                    continue;
                }
                _d = _d_in_loop;
                print!("i = {:?}, window_len = {:?}, new = true\n", i, window_len);
                continue;
            }

            if window_check {
                if window_len >= 3 {
                    _d = _d_in_loop;
                    let add = ((window_len - 2) * (window_len - 2 + 1)) / 2;
                    // print!("bf result = {:?} : ", result);
                    result += add;
                    window_len = 1;
                    print!(
                        "i = {:?}, window_len = {:?}, result = {:?}, add = {:?}\n",
                        i, window_len, result, add
                    );
                    new = true;
                    continue;
                }
                window_len = 1;
                new = true;
                print!("i = {:?}, window_len = {:?}\n", i, window_len);
            } else if !window_check {
                window_len += 1;
                print!("i = {:?}, window_len = {:?}\n", i, window_len);
            }

            // match _map.entry(window_count) {
            //     Entry::Occupied(mut _o) => {
            //         if *_o.get_mut() != 1 {
            //             // let _window = String::from_utf8(_bytes[(y - 9)..(y + 1)].to_vec()).unwrap();
            //             // result.push(_window);
            //             *_o.get_mut() = 1;
            //         }
            //     }
            //     Entry::Vacant(_v) => {
            //         _v.insert(2);
            //     }
            // }
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
