use std::fs;

struct Diff {
    main: usize,
    sub: usize,
    dist: isize
}

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        let parent = (0..size).collect();
        Dsu { parent }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        let root = self.find(self.parent[i]);
        self.parent[i] = root;
        root
    }

    fn union(&mut self, main: usize, sub: usize) -> bool {
        let root_main = self.find(main);
        let root_sub = self.find(sub);
        if root_main != root_sub {
            self.parent[root_main] = root_sub;
            return true;
        }
        false
    }
}

fn distance(point_main: &[&str], point_sub: &[&str]) -> isize {
    let point_main_value: Vec<isize> = point_main.iter().map(|point| point.parse::<isize>().unwrap()).collect();
    let point_sub_value: Vec<isize> = point_sub.iter().map(|point| point.parse::<isize>().unwrap()).collect();
    let diff_x: isize = point_sub_value[0] - point_main_value[0];
    let diff_y: isize = point_sub_value[1] - point_main_value[1];
    let diff_z: isize = point_sub_value[2] - point_main_value[2];
    diff_x.pow(2) + diff_y.pow(2) + diff_z.pow(2)
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("讀取失敗");
    let point_arr: Vec<Vec<&str>> = input
        .split("\n")
        .map(|point| point.split(",").collect())
        .collect();
    let point_arr_len: usize = point_arr.len();
    let mut dist_data_all: Vec<Diff> = Vec::new();

    for main in 0..point_arr_len {
        for sub in (main + 1)..point_arr_len {
            let point_main = &point_arr[main];
            let point_sub = &point_arr[sub];
            let diff_data = Diff {
                main,
                sub,
                dist: distance(point_main, point_sub),
            };
            dist_data_all.push(diff_data);
        }
    }

    dist_data_all.sort_unstable_by(|prev, next| prev.dist.cmp(&next.dist));

    let mut dsu = Dsu::new(point_arr.len());
    let mut edges_count: usize = 0;

    for dist_item in &dist_data_all {
        if dsu.union(dist_item.main, dist_item.sub) {
            edges_count += 1;

            if edges_count == point_arr_len - 1 {
                let point_x_main: usize = point_arr[dist_item.main][0].parse::<usize>().unwrap();
                let point_x_sub: usize = point_arr[dist_item.sub][0].parse::<usize>().unwrap();
                println!("{}", point_x_main * point_x_sub);
            }
        }
        
    }
}