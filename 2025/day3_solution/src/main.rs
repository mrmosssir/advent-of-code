use std::fs;

fn check_largest_joltage(str: &str) -> i64 {
  let mut _large_buffer: Vec<i8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let nums = str.chars().map(|char| char.to_digit(10).unwrap() as i8);
  let len = str.len();

  for (index, num) in nums.enumerate() {
    for i in 0..=11 {
      let last_position = if len - 1 - index >= 12 {0} else { 12 - (len - 1 - index) - 1 };
      if num > _large_buffer[i] && i >= last_position {
        _large_buffer[i] = num;
        _large_buffer[(i + 1)..].fill(0);
        break;
      } else if num > _large_buffer[i] && index >= len - 1 {
        _large_buffer[11] = num;
      }
    }

    // part - 1
    // if num > _large_buffer[0] && index < len - 1 {
    //   _large_buffer[0] = num;
    //   _large_buffer[1] = 0;
    // } else if num > _large_buffer[1] {
    //   _large_buffer[1] = num;
    // }
  }

  let mut result: i64 = 0;

  for i in 0..=11 {
    result += (_large_buffer[i] as i64) * 10_i64.pow(11 - i as u32);
  }

  return result;
}

fn main() {
  let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
  let mut total: i64 = 0;
  
  for line in input.lines() {
    total += check_largest_joltage(line) as i64;
  }

  println!("{}", total);
}