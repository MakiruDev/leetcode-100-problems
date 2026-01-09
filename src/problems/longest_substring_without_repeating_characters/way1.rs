pub struct Solution;
use super::super::func_run;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(_s: String) -> i32 {
        let _char: Vec<char> = _s.chars().collect();
        let char_len = _char.len();
        let mut max_sum = 0;
        let mut now_sum = 0;
        let mut _map = HashMap::new();
        let mut min_insert = 0;

        for i in 0..char_len {
            if !_map.contains_key(&_char[i]) {
                _map.insert(_char[i], i);
                now_sum += 1;
            } else {
                if now_sum > max_sum {
                    max_sum = now_sum;
                }

                let last_char = _map.get(&_char[i]).unwrap().clone();
                now_sum = (i - last_char) as i32;

                for _get_char in min_insert..last_char + 1 {
                    _map.remove(&_char[_get_char]);

                    if _get_char == last_char {
                        min_insert = _get_char + 1;
                    }
                }

                _map.insert(_char[i], i);
            }
        }

        if now_sum > max_sum {
            max_sum = now_sum;
        }

        max_sum
    }
}

pub fn solve(input_str: &str) {
    if input_str == "input-format" {
        println!("1 input -> String");
        println!("something like this : text1");
        println!("example : {}", "abcabcbb");
        println!(
            "warning : if input have special characters, please use \"\",  \"your input here\""
        );
        println!("something like this : \"-$#;)(*%\"");
        return;
    }

    let param1 = input_str.to_string();

    let output = &Solution::length_of_longest_substring(param1).to_string();

    func_run::solve_log(
        "Longest Substring Without Repeating Characters",
        input_str,
        output,
    );
}
