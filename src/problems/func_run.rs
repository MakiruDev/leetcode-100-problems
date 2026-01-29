pub struct Format;

impl Format {
    pub fn to_i32(index_dir: usize, input: &str) -> i32 {
        let parts: Vec<&str> = input.split(' ').collect();
        let start_value = parts[index_dir];

        let result = start_value.parse::<i32>().unwrap();

        result
    }

    pub fn to_string(index_dir: usize, input: &str) -> String {
        let parts: Vec<&str> = input.split(' ').collect();
        let start_value = parts[index_dir];

        let result = start_value.to_string();

        result
    }

    pub fn to_vec_i32(index_dir: usize, input: &str) -> Vec<i32> {
        let parts: Vec<&str> = input.split(' ').collect();
        let start_value = parts[index_dir];

        let result = start_value
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>().expect("invalid number"))
            .collect();

        result
    }

    // pub fn to_string(index_dir: usize, input: &str) -> String {
    //     let parts: Vec<&str> = input.split(' ').collect();
    //     let start_value = parts[index_dir];

    //     let result = start_value.to_string();

    //     result
    // }

    // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----
    // use to format the output
    // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----

    // get string for f64 input
    // pub fn get_f64(_start: f64) -> String {
    //     let result = _start.to_string();

    //     result
    // }

    // pub fn get_i32(_start: i32) -> String {
    //     let result = _start.to_string();

    //     result
    // }
}

pub fn solve_log(name_pb: &str, _input_str: &str, output: &str) {
    println!("Solve: {}", name_pb);
    println!("input  -> {}", _input_str);
    println!("output -> {}", output);
}

// pub fn sand_need_format(nums: i32, input_need: String, expl: String, warning: u8) {
//     let _type: Vec<&str> = input_need.split(' ').collect();
//     let _type_len = _type.len();
//     let mut input_type = "".to_string();
//     let mut example = "".to_string();
//     // let warning_char = warning.tos

//     let mut type_container: Vec<String> = vec![
//         "test1".to_string(),
//         "test2".to_string(),
//         "test2".to_string(),
//         "test2".to_string(),
//         "test2".to_string(),
//     ];

//     let type_container_len = type_container.len();

//     for i in 0..type_container_len {
//         for j in 0.._type_len {
//             if type_container[i] == _type[j] {
//                 example = format!("{}, {}", example, type_container[i]);
//             }
//         }

//         if i < _type_len {
//             input_type = format!("{}, {}", input_type, _type[i]);
//         }
//     }

//     println!("{:?} input -> ", nums);
//     println!("something like this : {}", input_type);
//     println!("example : {}", expl);

//     println!("warning : if input have special characters, please use \"\",  \"your input here\"");
//     println!("something like this : \"-$#;)(*%\"");
// }
