use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("讀取失敗");
    let points: Vec<Vec<isize>> = input
        .split('\n')
        .map(|point| point.split(',').map(|position| position.parse::<isize>().unwrap()).collect())
        .collect();
    let mut max_square_size: isize = 0;
    
    for main_index in 0..(points.len() - 1) {
        for sub_index in (main_index + 1)..points.len() {
            let main_point: &Vec<isize> = &points[main_index];
            let sub_point: &Vec<isize> = &points[sub_index];
            let area: isize = (main_point[0] - sub_point[0] + 1) * (main_point[1] - sub_point[1] + 1).abs();
            if area > max_square_size {
                max_square_size = area;
            }
        }
    }

    println!("{max_square_size}");
}