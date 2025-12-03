use std::fs;
use std::cmp::Reverse;


fn get_inputs() -> Vec<String> {
    let file_path = "inputs.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let col: Vec<String> = contents.trim_end().split("\n").into_iter().map(|s| s.to_string()).collect();

    col
}

fn calculate_joltage(bank: &String, digs: usize) -> u64 {
    let bank_len = bank.len();
    let mut bank_vec: Vec<_> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut joltage: u64 = 0;
    let mut start_idx = 0;
    for n in (0..digs).rev() {
        let (idx, bat_jolt) = bank_vec[start_idx..bank_len - n].iter().enumerate().min_by_key(|&(_, v)| Reverse(v)).unwrap();
        start_idx += idx + 1;
        let jolt: u64 = u64::from(*bat_jolt) * (10 as u64).pow(n as u32);
        joltage += jolt;
    }

    joltage
}

fn part1(input: Vec<String>) -> u64 {
    let mut result: u64 = 0;

    for num in input.iter() {
        result += calculate_joltage(num, 2);
    }
    result
}

fn part2(input: Vec<String>) -> u64 {
    let mut result: u64 = 0;

    for num in input.iter() {
        result += calculate_joltage(num, 12);
    }
    result
}
fn main() {
    let inputs: Vec<String> = get_inputs();

    let sol1 = part1(inputs.clone());
    println!("Solution for part1: {sol1}");
    let sol2 = part2(inputs.clone());
    println!("Solution for part2: {sol2}");
}

