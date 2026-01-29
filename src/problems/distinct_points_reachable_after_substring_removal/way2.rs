use std::collections::{HashMap, hash_map::Entry};

use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn distinct_points(_s: String, _k: i32) -> i32 {
        let _byte = _s.into_bytes();
        let _len = _byte.len();
        let usize_k = _k as usize;
        let mut x = 0;
        let mut y = 0;
        let mut _map: HashMap<[i32; 2], ()> = HashMap::new();

        for i in 0.._len {
            match _byte[i] {
                b'R' => x += 1,
                b'L' => x += -1,
                b'U' => y += 1,
                b'D' => y += -1,
                _ => {}
            };

            if i >= usize_k - 1 {
                match _map.entry([x, y]) {
                    Entry::Occupied(mut _o) => {}
                    Entry::Vacant(_v) => {
                        _v.insert(());
                    }
                }

                match _byte[i + 1 - usize_k] {
                    b'R' => x -= 1,
                    b'L' => x -= -1,
                    b'U' => y -= 1,
                    b'D' => y -= -1,
                    _ => {}
                };
            }
        }

        _map.len() as i32
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
