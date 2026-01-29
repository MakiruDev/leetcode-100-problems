use std::collections::HashSet;

use crate::problems::func_run;

pub struct Solution;

impl Solution {
    pub fn distinct_points(_s: String, _k: i32) -> i32 {
        let _byte = _s.into_bytes();
        let usize_k = _k as usize;
        let mut bit_x: u32 = 0;
        let mut bit_y: u32 = 0;
        let mut _hash = HashSet::with_capacity((_byte.len() + 1 - usize_k) / 2); // get medium speed, use low mem
        // let mut _hash = HashSet::with_capacity((_byte.len() + 1 - usize_k) * 8 / 7); // get high speed, use high mem

        for i in 0.._byte.len() {
            match _byte[i] {
                b'R' => bit_x = bit_x.wrapping_add(1),
                b'L' => bit_x = bit_x.wrapping_sub(1),
                b'U' => bit_y = bit_y.wrapping_add(1),
                b'D' => bit_y = bit_y.wrapping_sub(1),
                _ => continue,
            };

            if i >= usize_k - 1 {
                _hash.insert((bit_x as u64) << 32 | (bit_y as u64));

                match _byte[i + 1 - usize_k] {
                    b'R' => bit_x = bit_x.wrapping_sub(1),
                    b'L' => bit_x = bit_x.wrapping_add(1),
                    b'U' => bit_y = bit_y.wrapping_sub(1),
                    b'D' => bit_y = bit_y.wrapping_add(1),
                    _ => continue,
                };
            }
        }

        _hash.len() as i32
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
