// use std::fs;

// fn check_largest_joltage(str: &str) -> i64 {
//   let mut _large_buffer: Vec<i8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//   let nums = str.chars().map(|char| char.to_digit(10).unwrap() as i8);
//   let len = str.len();

//   for (index, num) in nums.enumerate() {
//     for i in 0..=11 {
//       let last_position = if len - 1 - index >= 12 {0} else { 12 - (len - 1 - index) - 1 };
//       if num > _large_buffer[i] && i >= last_position {
//         _large_buffer[i] = num;
//         _large_buffer[(i + 1)..].fill(0);
//         break;
//       } else if num > _large_buffer[i] && index >= len - 1 {
//         _large_buffer[11] = num;
//       }
//     }

//     // part - 1
//     // if num > _large_buffer[0] && index < len - 1 {
//     //   _large_buffer[0] = num;
//     //   _large_buffer[1] = 0;
//     // } else if num > _large_buffer[1] {
//     //   _large_buffer[1] = num;
//     // }
//   }

//   let mut result: i64 = 0;

//   for i in 0..=11 {
//     result += (_large_buffer[i] as i64) * 10_i64.pow(11 - i as u32);
//   }

//   return result;
// }

// fn main() {
//   let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
//   let mut total: i64 = 0;
  
//   for line in input.lines() {
//     total += check_largest_joltage(line) as i64;
//   }

//   println!("{}", total);
// }

use std::fs;

const TARGET_LEN: usize = 12;

fn solve_largest_subsequence(input_str: &str) -> i64 {
    // 1. 將字串轉為數字陣列
    let digits: Vec<u32> = input_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let total_len = digits.len();
    
    // 如果輸入長度不足 12，直接回傳 0 或做其他處理
    if total_len < TARGET_LEN {
        return 0;
    }

    let mut stack: Vec<u32> = Vec::with_capacity(TARGET_LEN);

    for (i, &digit) in digits.iter().enumerate() {
        // 計算這一輪結束後，後面還有多少個數字可用
        let remaining_digits = total_len - 1 - i;

        // 單調棧核心邏輯
        while let Some(&last) = stack.last() {
            // 判斷是否要替換：
            // 1. 新數字比 Stack 頂端的大 (digit > last)
            // 2. 替換後，我們仍然有足夠的數字可以湊滿 12 位
            //    公式：(Stack目前長度 - 1 個被丟的 + 1 個現在手上的) + 剩餘的 >= 目標長度
            //    簡化後：Stack目前長度 + 剩餘的 >= 目標長度
            if digit > last && (stack.len() + remaining_digits >= TARGET_LEN) {
                stack.pop();
            } else {
                break;
            }
        }

        // 如果 Stack 還沒滿，就把當前數字塞進去
        if stack.len() < TARGET_LEN {
            stack.push(digit);
        }
    }

    // 將 Stack 內的數字組合成 i64 (例如 [9, 8, 7] -> 987)
    stack.iter().fold(0i64, |acc, &d| acc * 10 + d as i64)
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
    
    let total: i64 = input
        .lines()
        .map(|line| solve_largest_subsequence(line))
        .sum();

    println!("{}", total);
}