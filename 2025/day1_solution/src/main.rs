use std::fs;

fn main() {
    // 讀取檔案，這裡直接 unwrap 處理錯誤（AoC 解題通常求快）
    let input = fs::read_to_string("src/input.txt").expect("無法讀取檔案");
    
    let mut scale: i32 = 50;
    let mut answer: i32 = 0;

    for line in input.lines() {
        let line = line.trim();
        // 過濾空行
        if line.is_empty() { continue; }

        // 切割字串：前 1 個字元是方向，後面是數值
        let (dir, val_str) = line.split_at(1);
        
        // 解析數值
        let val: i32 = val_str.parse().expect("數值解析失敗");

        match dir {
            "R" => {
                let raw = scale + val;
                
                // JS: Math.floor(raw / 100)
                // Rust: 整數除法 150 / 100 = 1，自動無條件捨去，效果等同 Math.floor
                answer += raw / 100;
                
                // 安全取餘數 ((n % m) + m) % m
                scale = ((raw % 100) + 100) % 100;
            },
            "L" => {
                let raw = scale - val;
                
                if raw <= 0 {
                    // JS: answer += Math.abs(Math.ceil(raw / 100) - 1);
                    // Rust: 透過整數除法特性簡化邏輯
                    // 邏輯推導：
                    // raw = -1   -> JS ceil(-0.01)=-0 -> abs(-1)=1 | Rust -1/100=0   -> abs(0)+1=1
                    // raw = -100 -> JS ceil(-1.0)=-1  -> abs(-2)=2 | Rust -100/100=-1 -> abs(1)+1=2
                    // raw = -101 -> JS ceil(-1.01)=-1 -> abs(-2)=2 | Rust -101/100=-1 -> abs(1)+1=2
                    // raw = -200 -> JS ceil(-2.0)=-2  -> abs(-3)=3 | Rust -200/100=-2 -> abs(2)+1=3
                    // 結論：(raw / 100).abs() + 1 完美等價
                    let cross = if scale > 0 { 1 } else { 0 };
                    answer += (raw / 100).abs() + cross;
                }
                
                // 安全取餘數
                scale = ((raw % 100) + 100) % 100;
            },
            _ => panic!("未知的方向: {}", dir),
        }
    }

    println!("{}", answer);
}