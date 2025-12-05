use std::fs;

fn get_inputs() -> (Vec<String>, Vec<u64>) {
    let file_path = "inputs.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let col: Vec<String> = contents
        .split_whitespace()
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let ranges: Vec<String> = col
        .iter()
        .filter(|&part| part.contains('-'))
        .cloned()
        .collect();
    let numbers: Vec<u64> = col
        .iter()
        .filter(|&part| !part.contains('-'))
        .filter_map(|s| s.parse().ok())
        .collect();
    (ranges, numbers)
}

fn part1(id_ranges: Vec<String>, ids: Vec<u64>) -> u64 {
    let mut result = 0;

    for id in ids.iter() {
        for range in id_ranges.iter() {
            let i_r: Vec<_> = range
                .split("-")
                .into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            if *id <= i_r[1] && *id >= i_r[0] {
                result += 1;
                break;
            }
        }
    }
    result
}

fn part2(id_ranges: Vec<String>) -> u64 {
    let mut result = 0;
    let mut ranges: Vec<(u64, u64)> = id_ranges
        .iter()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect();
    ranges.sort_by_key(|&(start, _)| start);
    let mut merged = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 {
            // Overlapping or adjacent, merge them
            current.1 = current.1.max(end);
        } else {
            // No overlap, push current and start a new range
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    for (start, end) in merged.iter() {
        result += end - start + 1
    }
    result
}
fn main() {
    let (i_ranges, i_numbers) = get_inputs();

    let sol1 = part1(i_ranges.clone(), i_numbers.clone());
    println!("Solution for part1 {:?}", sol1);
    let sol2 = part2(i_ranges.clone());
    println!("Solution for part2 {:?}", sol2);
}
