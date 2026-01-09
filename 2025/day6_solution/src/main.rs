use std::fs;

fn exec_calculate (str_arr: &Vec<String>, formula: char) -> usize {
    let mut result: usize = if formula == '*' { 1 } else { 0 };
    for s in str_arr.iter() {
        match formula {
            '+' => {
                result += s.parse::<usize>().unwrap_or(0);
            },
            '*' => {
                result *= s.parse::<usize>().unwrap_or(1);
            },
            _ => {}
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
    let lines: Vec<&str> = input.lines().collect();
    let sum_group:Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let formula_index: usize = 4;

    let max_len: usize = sum_group
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);
    let start_position: usize = max_len - 1;

    let sum_group_len: usize = sum_group.len();
    let mut total: usize = 0;
    let mut total_buffer: Vec<String> = Vec::new();

    for position in (0..=start_position).rev() {
        let mut num_str: String = String::new();
        for idx in 0..(sum_group_len - 1) {
            if sum_group[idx][position] != ' ' {
                num_str.push(sum_group[idx][position]);
            }
        }
        
        if num_str != "" {
            total_buffer.push(num_str);
        }
        
        if sum_group[formula_index][position] != ' ' {
            total += exec_calculate(&total_buffer, sum_group[formula_index][position]);
            total_buffer = Vec::new();
        }
    }
    println!("{total}")
}