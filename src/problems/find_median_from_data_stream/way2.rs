use crate::problems::func_run;

use std::collections::VecDeque;

pub struct MedianFinder {
    vec: VecDeque<i32>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            vec: VecDeque::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        let pos = self.vec.binary_search(&num).unwrap_or_else(|p| p);
        self.vec.insert(pos, num);
    }

    pub fn find_median(&self) -> f64 {
        let index = self.vec.len() >> 1;

        if self.vec.len() & 1 == 1 {
            f64::from(self.vec[index])
        } else {
            (f64::from(self.vec[index - 1]) + f64::from(self.vec[index])) / 2_f64
        }
    }
}

pub fn solve(input_str: &str) {
    if input_str == "input-format" {
        println!("2 input -> MedianFinder,addNum,addNum,findMedian,addNum,findMedian 1,2,3");
        println!("1 output -> null,null,null,1.5,null,2.0");
        println!(
            "example : MedianFinder,addNum,addNum,findMedian,addNum,findMedian 1,2,3 => null,null,null,1.5,null,2.0"
        );
        return;
    }

    let param1 = func_run::Format::to_string(0, input_str);
    let param2 = func_run::Format::to_vec_i32(1, input_str);

    let operations: Vec<&str> = param1.split(',').collect();
    let mut nums = param2.into_iter();

    let mut obj = MedianFinder::new();
    let mut results: Vec<String> = vec![];

    for op in operations {
        match op {
            "MedianFinder" => {
                obj = MedianFinder::new();
                results.push("null".to_string());
            }
            "addNum" => {
                let num = nums.next().expect("addNum needs one more number in param2");
                obj.add_num(num);
                results.push("null".to_string());
            }
            "findMedian" => {
                let ret: f64 = obj.find_median();
                results.push(format!("{:?}", ret));
            }
            _ => panic!("unknown operation: {}", op),
        }
    }

    let output = &results.join(",");

    func_run::solve_log("find_median", input_str, output);
}
