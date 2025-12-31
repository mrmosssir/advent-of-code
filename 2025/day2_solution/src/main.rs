use std::fs;

fn check_repeat(str: &str) -> bool {
  let double_str: String = str.repeat(2);
  // 驗證字串去頭去尾
  let mut valid_chars = double_str.chars();
  valid_chars.next();
  valid_chars.next_back();
  let valid_str: String = valid_chars.collect();
  
  return valid_str.contains(str)
} 

fn main() {
  let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
  let arr: Vec<&str> = input.split(",").collect();
  let mut result: i64 = 0;

  for i in 0..arr.len() {
    let values: Vec<i64> = arr[i].split("-").map(|s| s.trim().parse().unwrap()).collect();
    for num in values[0]..=values[1] {
      if check_repeat(&num.to_string()) {
        result += num;
      }
    }
  }

  println!("{}", result);
}