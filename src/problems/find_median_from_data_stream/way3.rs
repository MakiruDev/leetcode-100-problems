use crate::problems::func_run;

struct Fenwick {
    tree: Vec<u16>,
    // len: usize,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn add(&mut self, num: i32) {
        let mut index = (num + 100001) as i32;

        while index.trailing_zeros() <= 18 {
            self.tree[index as usize] += 1;
            index += 1 << index.trailing_zeros();
        }
    }

    fn length(&self) -> usize {
        self.tree[262144_usize] as usize
    }

    fn prefix_sum(&self, num: i32) -> u16 {
        let mut index = num + 100001;
        let mut sum = 0;

        loop {
            sum += self.tree[index as usize];
            index -= 1 << index.trailing_zeros();

            if index == 0 {
                break;
            }
        }
        sum
    }

    fn binary_search(&self) -> f64 {
        let target = ((self.length() >> 1) + 1) as u16;
        let target2 = target - 1;
        let mut expo = 17;
        loop {
            if self.tree[1 << expo] < target {
                // 2^expo + 1, 2^(expo + 1)
                break;
            } else {
                expo -= 1;
                if expo < 0 {
                    // target is in range -100,000
                    // return (-100000, self.tree[1]);
                    return -100000f64;
                }
            }
        }
        loop {
            // 2^expo + 1, 2^(expo + 1)
            if expo == 0 {
                // target is tree[1]
                if self.length() & 1 == 0 {
                    todo!()
                } else {
                    return -99999f64;
                }
            }
            let range = 1 << (expo - 1);
        }
        // // if self.length() & 1 == 0 {}
        // let num = (1 << expo) - 100001;
        // let amount = self.tree[1 << expo];

        // (num, amount)
        todo!()
    }
}

pub struct MedianFinder {
    fenwick: Fenwick,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            fenwick: Fenwick::new(300000),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.fenwick.add(num);
    }

    pub fn find_median(&self) -> f64 {
        // self.fenwick
        todo!()
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
