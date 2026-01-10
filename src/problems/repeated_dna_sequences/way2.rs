use crate::problems::func_run;
use std::collections::HashMap;
use std::string::String;
use std::u8;

pub struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(_s: String) -> Vec<String> {
        let _bytes = _s.into_bytes();
        let _len = _bytes.len();
        let mut _map: HashMap<Vec<u8>, bool> = HashMap::new();
        let mut result: Vec<String> = std::vec![];

        if _len < 11 as usize {
            return result;
        }

        for x in 10.._len + 1 {
            let _window = _bytes[(x - 10)..x].to_vec();
            let _val = _map.get(&_window).copied().unwrap_or(false);

            // print!("has {} : {} \n", _window, _map.contains_key(&_window));
            // print!("val {} : {} \n", _window, _val);
            if _map.contains_key(&_window) {
                if _val {
                    // String::from_utf8(_bytes[(x - 10)..x].to_vec()).unwrap()
                    let _window_string = String::from_utf8(_window.clone()).unwrap();
                    result.push(_window_string);
                    _map.insert(_window, false);
                }
            } else {
                _map.insert(_window, true);
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
