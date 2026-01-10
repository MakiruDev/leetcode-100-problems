use crate::problems::func_run;
use std::collections::{HashMap, hash_map::Entry};
use std::string::String;

pub struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(_s: String) -> Vec<String> {
        let _bytes = _s.into_bytes();
        let _len = _bytes.len();
        let mut _map: HashMap<u32, bool> = HashMap::with_capacity(_len.saturating_sub(9));
        let mut result: Vec<String> = std::vec![];
        let mut code: u32 = 0;

        if _len < 11 as usize {
            return result;
        }

        for y in 0.._len {
            let val = match _bytes[y] {
                b'A' => 0,
                b'C' => 1,
                b'G' => 2,
                b'T' => 3,
                _ => continue,
            };

            code = (code << 2) | val;
            code &= (1 << 20) - 1;

            if y > 8 {
                match _map.entry(code) {
                    Entry::Occupied(mut _o) => {
                        if *_o.get_mut() {
                            let _window =
                                String::from_utf8(_bytes[(y - 9)..(y + 1)].to_vec()).unwrap();
                            result.push(_window);
                            *_o.get_mut() = false;
                        }
                    }
                    Entry::Vacant(_v) => {
                        _v.insert(true);
                    }
                }
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
