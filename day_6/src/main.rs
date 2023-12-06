use std::{fs::read_to_string};


fn main() {
    let data = read_to_string("src/input.txt").unwrap();
    let mut lines = data.lines();

    let mut time_line = lines.next().unwrap().split_whitespace();time_line.next();
    let times: Vec<i32> = time_line.map(|s| s.parse::<i32>().unwrap()).into_iter().collect();

    let mut distance_line = lines.next().unwrap().split_whitespace();distance_line.next();
    let distances: Vec<i32> = distance_line.map(|s| s.parse::<i32>().unwrap()).into_iter().collect();

    let mut prod = 1;

    for i in 0..times.len() {
        let mut sum = 0;
        for j in 1..times[i] {
            if j * (times[i] - j) > distances[i] {
                sum += 1;
            }
        }
        prod *= sum;
    }

    println!("{}", prod);
}
