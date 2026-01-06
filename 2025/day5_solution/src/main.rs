
use std::fs;

fn split_range(range: &str) -> (usize, usize) {
    let range_parts: Vec<&str> = range.split('-').collect();
    let start: usize = range_parts[0].parse().unwrap();
    let end: usize = range_parts[1].parse().unwrap();
    return (start, end);
}

fn check_is_stack(range: (usize, usize), range_target: (usize, usize)) -> bool {
    if (range.0 < range_target.0 && range.1 < range_target.0) || (range.0 > range_target.1 && range.1 > range_target.1) {
        return false;
    }
    return true;
}

fn find_max_range(range: (usize, usize), range_target: (usize, usize)) -> (usize, usize) {
    let all_range_point = vec![range.0, range.1, range_target.0, range_target.1];
    let min = all_range_point.iter().min().unwrap();
    let max = all_range_point.iter().max().unwrap();
    return (*min, *max);
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
    let groups = input
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    // let mut counter: u16 = 0;
    let mut freshs = <Vec<(usize, usize)>>::new();

    for range_item in &groups[0] {
        let (start, end) = split_range(range_item);
        let mut new_range = (start, end);
        let mut indices_to_remove = Vec::new();
        
        for (index, fresh) in freshs.iter().enumerate() {
            if check_is_stack((new_range.0, new_range.1), (fresh.0, fresh.1)) {
                new_range = find_max_range((new_range.0, new_range.1), (fresh.0, fresh.1));
                indices_to_remove.push(index);
            }
        }
        
        // 刪除重疊的範圍
        for index in indices_to_remove.iter().rev() {
            freshs.remove(*index);
        }
        
        freshs.push(new_range);
    }

    let mut total = 0;

    for fresh in &freshs {
        total += fresh.1 - fresh.0 + 1;
    }

    // for number_str in &groups[1] {
    //     let number = number_str.parse::<usize>().unwrap();
    //     for range_item in &groups[0] {
    //         let (start, end) = split_range(range_item);
    //         if number >= start && number <= end {
    //             counter += 1;
    //             break;
    //         }
    //     }
    // }

    println!("{}", total);
}