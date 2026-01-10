use crate::problems::func_run;
use std::collections::HashMap;
use std::string::String;

pub struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(_s: String) -> Vec<String> {
        let _char: Vec<char> = _s.chars().collect();
        let _len = _char.len();
        let mut _map: HashMap<String, i32> = HashMap::new();
        let mut result: Vec<String> = std::vec![];

        if _len < 11 as usize {
            return result;
        }

        for x in 10.._len + 1 {
            let _window: String = _char[(x - 10)..=(x - 1)].iter().collect();
            let _val = _map.get(&_window).copied().unwrap_or(0);
            let condition = _val < 2;
            // print!("{} : {} \n", _window, _map.contains_key(&_window));
            if _map.contains_key(&_window) {
                if condition {
                    result.push(_window.clone());
                    _map.insert(_window, 2);
                } else {
                    _map.insert(_window, _val + 1);
                }
            } else {
                _map.insert(_window, 1);
            }
        }

        result
    }
}

pub fn solve(input_str: &str) {
    if input_str == "input-format" {
        println!("1 input -> String");
        println!("something like this : \"String\"");
        println!("example : {}", "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
        return;
    }

    let input_string = input_str.to_string();

    let output = &Solution::find_repeated_dna_sequences(input_string).join("-");

    func_run::solve_log("Repeated DNA Sequences", input_str, output);
}
