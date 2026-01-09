use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("讀取失敗");
    let lines: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let smoke_size: usize = lines[0].len();

    let some_start_position = lines[0].iter().position(|&c| c == 'S');
    let mut tachyon_position: Vec<usize> = Vec::new();

    let mut split_times: usize = 0;

    match some_start_position {
        Some(i) => tachyon_position.push(i),
        None => return
    }

    for line in 1..lines.len() {
        let mut buffer_tachyon_position: Vec<usize> = Vec::new();
        for position in tachyon_position.iter() {
            if lines[line][*position] == '^' {
                split_times += 1;
                let prev = position.saturating_sub(1);
                let next = (position + 1).min(smoke_size - 1);
                if !buffer_tachyon_position.contains(&prev) {
                    buffer_tachyon_position.push(prev);
                }
                if !buffer_tachyon_position.contains(&next) {
                    buffer_tachyon_position.push(next);
                }
            } else if !buffer_tachyon_position.contains(&position) {
                buffer_tachyon_position.push(*position);
            }
            
        }
        if buffer_tachyon_position.len() > 0 {
            tachyon_position = buffer_tachyon_position;
        }
    }

    println!("{split_times}");
}