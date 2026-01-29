use std::collections::{HashMap, VecDeque, hash_map::Entry};

use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn distinct_points(_s: String, _k: i32) -> i32 {
        let _byte = _s.into_bytes();
        let _len = _byte.len();
        let usize_k = _k as usize;
        let mut result = 0;
        let mut x = 0;
        let mut y = 0;
        let mut _map: HashMap<[i32; 2], ()> = HashMap::new();
        let mut value_save = VecDeque::new();

        for i in 0.._len {
            match _byte[i] {
                b'R' => {
                    value_save.push_back([1, 0]);
                    x += 1;
                }
                b'L' => {
                    value_save.push_back([-1, 0]);
                    x += -1;
                }
                b'U' => {
                    value_save.push_back([0, 1]);
                    y += 1;
                }
                b'D' => {
                    value_save.push_back([0, -1]);
                    y += -1;
                }
                _ => {}
            };

            if i >= usize_k - 1 {
                match _map.entry([x, y]) {
                    Entry::Occupied(mut _o) => {}
                    Entry::Vacant(_v) => {
                        _v.insert(());
                        result += 1;
                    }
                }

                let _xy = value_save.pop_front().unwrap();
                x -= _xy[0];
                y -= _xy[1];
            }
        }

        result
    }
}

pub fn solve(input_str: &str) {
    if input_str == "input-format" {
        println!("2 input -> UDLLLLRDLDRRD 3");
        println!("1 output -> 6");
        println!("example : UDLLLLRDLDRRD 3 => 6");
        return;
    }

    let param1 = func_run::Format::to_string(0, input_str);
    let param2 = func_run::Format::to_i32(1, input_str);

    let output = &Solution::distinct_points(param1, param2).to_string();

    func_run::solve_log("distinct_points", input_str, output);
}
