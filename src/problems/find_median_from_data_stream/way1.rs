use crate::problems::func_run;
use std::collections::BTreeMap;

pub struct MedianFinder {
    map: BTreeMap<i32, usize>,
    len: u16,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn add_num(&mut self, _num: i32) {
        // self.map.insert(k, v);
        self.map.entry(_num).and_modify(|v| *v += 1).or_insert(1);
        self.len += 1;
    }

    pub fn find_median(&self) -> f64 {
        let index = self.len >> 1;

        let keys: Vec<i32> = self.map.keys().copied().collect();
        let counts: Vec<usize> = self.map.values().copied().collect();

        let hit = counts
            .iter()
            .scan(0, |sum, &x| {
                *sum += x;
                Some(*sum)
            })
            .enumerate()
            .find(|&(_, s)| s as u16 >= index);

        let (pos, s) = hit.unwrap();

        if self.len & 1 == 0 {
            if index + 1 > s as u16 {
                return (keys[pos] + keys[pos + 1]) as f64 / 2 as f64;
            } else {
                return keys[pos] as f64;
            }
        } else {
            if index + 1 > s as u16 {
                return keys[pos + 1] as f64;
            } else {
                return keys[pos] as f64;
            }
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
