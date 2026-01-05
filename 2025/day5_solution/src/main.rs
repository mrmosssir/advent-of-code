
use std::fs;

fn split_range(range: &str) -> (usize, usize) {
    let range_parts: Vec<&str> = range.split('-').collect();
    let start: usize = range_parts[0].parse().unwrap();
    let end: usize = range_parts[1].parse().unwrap();
    return (start, end);
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
    let groups = input
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut counter: u16 = 0;

    for number_str in &groups[1] {
        let number = number_str.parse::<usize>().unwrap();
        for range_item in &groups[0] {
            let (start, end) = split_range(range_item);
            if number >= start && number <= end {
                counter += 1;
                break;
            }
        }
    }

    println!("{}", counter);
}