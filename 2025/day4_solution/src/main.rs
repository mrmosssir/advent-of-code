use std::fs;

// 獲取周圍座標
fn get_around_position(x: i16, y: i16, size_x: i16, size_y: i16) -> Vec<[i16; 2]> {
    let positions = [
        [x - 1, y - 1],             // Top-Left
        [x, y - 1],                 // Top
        [x + 1, y - 1],             // Top-Right
        [x - 1, y],                 // Left
        [x + 1, y],                 // Right
        [x - 1, y + 1],             // Bottom-Left
        [x, y + 1],                 // Bottom
        [x + 1, y + 1],             // Bottom-Right
    ];

    // 過濾掉有負作標的位置
    let filter_positions: Vec<[i16; 2]> = positions.iter().filter_map(|pos| {
        if pos[0] >= 0 && pos[1] >= 0 && pos[0] < size_x && pos[1] < size_y {
            Some(*pos)
        } else {
            None
        }
    }).collect();

    return filter_positions;
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
    let mut arr: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let width = arr[0].len();
    let height = arr.len();
    let mut current_round_can_moving: i32 = 0;
    let mut total_can_moving: i32 = 0;

    while !(total_can_moving > 0 && current_round_can_moving == 0) {
        // 初始化該輪搬移的紙卷數
        let mut will_moving_positions: Vec<[i16; 2]> = Vec::new();
        current_round_can_moving = 0;

        for y in 0..height {
            for x in 0..width {
                
                // 當前位置沒有紙的話跳過
                if arr[y][x] == '.' {
                    continue;
                }
                let mut count: i8 = 0;
                let positions = get_around_position(x as i16, y as i16, width as i16, height as i16);
                
                // 確認周圍卷數
                for pos in positions {
                    let check_x = pos[0] as usize;
                    let check_y = pos[1] as usize;
                    if arr[check_y][check_x] == '@' {
                        count += 1;
                    }
                }
                // 周圍小於 4 卷表示可以拿掉
                if count < 4 {
                    current_round_can_moving += 1;
                    will_moving_positions.push([x as i16, y as i16]);
                }
            }
        }
        total_can_moving += current_round_can_moving;

        for pos in will_moving_positions {
            // 標記為已搬移
            arr[pos[1] as usize][pos[0] as usize] = '.';
        }
    }

    
    println!("{}", total_can_moving);
}