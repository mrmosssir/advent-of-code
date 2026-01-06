use std::fs;

fn str_to_arr(num_str: &str) -> Vec<u16> {
    let str_arr: Vec<&str> = remove_space(num_str);
    let mut num_arr: Vec<u16> = Vec::new();
    for str_item in str_arr {
        let num: u16 = str_item.parse().unwrap();
        num_arr.push(num);
    }
    return num_arr;
}

fn remove_space(source: &str) -> Vec<&str> {
    source.split_whitespace().collect()
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
    let lines: Vec<&str> = input.lines().collect();
    let mut sum_group:Vec<Vec<u16>> = Vec::new();
    let mut sum_result_group:Vec<usize> = Vec::new();

    sum_group.push(str_to_arr(lines[0]));
    sum_group.push(str_to_arr(lines[1]));
    sum_group.push(str_to_arr(lines[2]));
    sum_group.push(str_to_arr(lines[3]));
    let formula_group:Vec<&str> = remove_space(lines[4]);

    for num in 0..sum_group[0].len() {
        let sum: usize = match formula_group[num] {
            "+" => {
                (sum_group[0][num] as usize) + (sum_group[1][num] as usize) + (sum_group[2][num] as usize) + (sum_group[3][num] as usize)
            },
            "*" => {
                (sum_group[0][num] as usize) * (sum_group[1][num] as usize) * (sum_group[2][num] as usize) * (sum_group[3][num] as usize)
            },
            _ => 0,
        };
        println!("第 {} 個結果: {}", num + 1, sum);
        sum_result_group.push(sum);
    }

    let sum: usize = sum_result_group.iter().sum();
    println!("結果: {}", sum);
}