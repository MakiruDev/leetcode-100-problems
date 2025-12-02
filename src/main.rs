mod problems;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // cargo run -- <problem> <way> <inputStr>
    if args.len() < 4 {
        eprintln!("Usage: cargo run -- <problem> <way> <inputStr>");
        return;
    }

    let problem = &args[1];
    let way = &args[2];
    let input_str = args[3..].join(" ");

    match (problem.as_str(), way.as_str()) {
        // problems 1
        ("1", "way1") => problems::maximum_average_subarray::way1::solve(&input_str),
        ("1", "way2") => problems::maximum_average_subarray::way2::solve(&input_str),

        // problems 1
        ("2", "way1") => {
            problems::longest_substring_without_repeating_characters::way1::solve(&input_str)
        }
        ("2", "way2") => {
            problems::longest_substring_without_repeating_characters::way2::solve(&input_str)
        }
        // ("1", "way3") => problems::maximum_average_subarray::way2::solve(&input_str),
        _ => eprintln!("Not found: problem {} way {}", problem, way),
    }
}
