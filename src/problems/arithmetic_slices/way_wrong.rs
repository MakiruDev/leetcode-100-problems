use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(_nums: Vec<i32>) -> i32 {
        let mut window_len = 1;
        let mut window_len_back = 1;
        let mut window_num = 0;
        let mut we_in = "front";
        let mut _len = _nums.len();
        let mut result = 0;

        let loop_skip = false;

        // let wid_to_num = |_wind: i32| ((_wind - 2) * (_wind - 2 + 1)) / 2;
        fn window_clear(len_of_window: &mut i32, aws: &mut i32) {
            let add = ((*len_of_window - 2) * (*len_of_window - 2 + 1)) / 2;
            *aws += add;
            *len_of_window = 1;
        }

        if _len < 3 {
            return result;
        }

        for i in 0.._len {
            let now = _nums[i];
            let to_front = window_num + 1 == now || window_num == now;
            let to_back = window_num - 1 == now;

            if loop_skip {}

            if now == 7 {
                print!(
                    "is 7, to_front = {:?}, to_back = {:?}, we_in = {:?}, window_num = {:?}, now = {:?}, ln\n",
                    to_front, to_back, we_in, window_num, now
                );
            } else {
                print!("window_num = {:?}, now = {:?}, : ", window_num, now);
            }

            if i == 0 || (!to_front && !to_back) {
                window_num = now;

                if window_num + 1 == _nums[i + 1] || window_num == _nums[i + 1] {
                    print!("i == 0 (window pass front) \n");
                    we_in = "front";
                    window_len = 1;
                } else if window_num - 1 == _nums[i + 1] {
                    print!("i == 0 (window pass back) \n");
                    we_in = "back";
                    window_len_back = 1;
                } else if _nums[i + 1] + 1 == _nums[i + 2] || _nums[i + 1] == _nums[i + 2] {
                    print!("i == 7 (exc window pass front) \n");
                    we_in = "front";
                    window_len += 1;
                } else if _nums[i + 1] - 1 == _nums[i + 2] {
                    print!("i == 7 (exc window pass back) \n");
                    we_in = "back";
                    window_len_back += 1;
                }

                if (!to_front && !to_back) && i != 0 {
                    if window_len_back != 1 {
                        print!("exc back if && clear");
                        window_clear(&mut window_len_back, &mut result);
                    } else if window_len != 1 {
                        print!("exc front if && clear");
                        window_clear(&mut window_len, &mut result);
                    }
                }
                continue;
            } else if to_front && we_in == "front" {
                print!("i != 0 (window pass front) \n");
                window_num = now;
                window_len += 1;
                // we_in = "front";
            } else if to_back && we_in == "back" {
                print!("i != 0 (window pass back) \n");
                window_num = now;
                window_len_back += 1;
                // we_in = "back";
            }

            // window_exit
            if to_front && we_in == "back" {
                // come from back
                print!("i != 0 (not pass front) \n");
                if window_len_back < 3 {
                    window_len_back = 1;
                } else {
                    // let add = wid_to_num(window_len_back);
                    // result += add;
                    // window_len_back = 1;
                    window_clear(&mut window_len_back, &mut result);
                }

                if to_front {
                    window_len += 1;
                }
                window_num = now;
                we_in = "front";
            } else if to_back && we_in == "front" {
                // come from front
                print!("i != 0 (not pass back) \n");
                if window_len < 3 {
                    window_len = 1;
                } else {
                    // let add = wid_to_num(window_len);
                    // result += add;
                    // window_len = 1;
                    window_clear(&mut window_len, &mut result);
                }

                // print!("bf window_len_back = {:?} \n", window_len_back);
                if to_back {
                    window_len_back += 1;
                }
                // print!("af window_len_back = {:?} \n", window_len_back);
                window_num = now;
                we_in = "back";
            }
        }

        if window_len > 2 {
            // let add_window_len = wid_to_num(window_len);
            // result += add_window_len;
            window_clear(&mut window_len, &mut result);
            print!("exit! front \n");
        } else if window_len_back > 2 {
            // let add_window_len_back = wid_to_num(window_len_back);
            // result += add_window_len_back;
            window_clear(&mut window_len_back, &mut result);
            print!("exit! back \n");
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
