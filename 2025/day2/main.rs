use std::fs;
use fancy_regex::Regex;

fn get_inputs() -> Vec<String> {
    let file_path = "inputs.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let col: Vec<String> = contents.trim_end().split(",").into_iter().map(|s| s.to_string()).collect();

    col
}

fn part1(input: Vec<String>) -> u64 {
    let mut result: u64 = 0;
    for range in input.iter() {
        let r1: Vec<_> = range.split("-").into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
        let vec: Vec<u64> = (r1[0]..r1[1]+1).collect();
        for nr in vec.iter() {
            let s = nr.to_string();
            let len = s.len();
            if len % 2 != 0 {
                continue;
            }
            let half = len / 2;

            let lower = s[..half].parse::<u64>().unwrap();
            let upper = s[half..].parse::<u64>().unwrap();

            if lower == upper {
                result += nr;
            }
        }
    }
    result
}

fn part2(input: Vec<String>) -> u64 {
    let mut result: u64 = 0;
    let matching_pattern = Regex::new(r"^(.+)\1+$").unwrap();
    for range in input.iter() {
        let r1: Vec<_> = range.split("-").into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
        let vec: Vec<u64> = (r1[0]..r1[1]+1).collect();
        for nr in vec.iter() {
            let s = nr.to_string();
            let is_match = matching_pattern.is_match(&s);
            if is_match.unwrap() {
                result += nr;
            }
        }
    }
    result
}

fn main() {
    let inputs: Vec<String> = get_inputs();

    let sol1: u64 = part1(inputs.clone());
    println!("Solution1 result: {sol1}");
    let sol2: u64 = part2(inputs);
    println!("Solution1 result: {sol2}");
}
