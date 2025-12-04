use std::fs;

fn get_inputs() -> Vec<Vec<char>> {
    let file_path = "inputs.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = contents
        .trim_end()
        .split("\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    grid
}

fn count_neighbours(grid: Vec<Vec<char>>, index_x: usize, index_y: usize) -> u64 {
    let mut counter = 0;
    let x_0 = index_x.saturating_sub(1);
    let y_0 = index_y.saturating_sub(1);
    let y_1 = index_y + 1;
    let x_1 = index_x + 1;
    for y in y_0..=y_1 {
        for x in x_0..=x_1 {
            if (y, x) == (index_y, index_x) {
                continue;
            }
            if y >= grid.len() || x >= grid[0].len() {
                continue;
            }
            if matches!(grid[y][x], '@') {
                counter += 1
            }
        }
    }
    counter
}
fn part1(grid: Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    let len_y = grid.len();
    let len_x = grid[0].len();
    for y in 0..len_y {
        for x in 0..len_x {
            if !matches!(grid[y][x], '@') {
                continue;
            }
            let counter = count_neighbours(grid.clone(), x, y);
            if counter < 4 {
                result += 1;
            }
        }
    }
    result
}

fn part2(mut grid: Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    let len_y = grid.len();
    let len_x = grid[0].len();
    loop {
        let mut tmp = 0;
        for y in 0..len_y {
            for x in 0..len_x {
                if !matches!(grid[y][x], '@') {
                    continue;
                }
                let counter = count_neighbours(grid.clone(), x, y);
                if counter < 4 {
                    tmp += 1;
                    grid[y][x] = '.';
                }
            }
        }
        if tmp == 0 {
            break;
        }
        result += tmp;
    }
    result
}
fn main() {
    let inputs: Vec<Vec<char>> = get_inputs();

    let sol1 = part1(inputs.clone());
    println!("Solution for part1 {:?}", sol1);
    let sol2 = part2(inputs.clone());
    println!("Solution for part2 {:?}", sol2);
}
