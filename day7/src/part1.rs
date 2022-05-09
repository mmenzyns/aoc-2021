use std::fs;

pub fn main() {
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    let heights: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>()
            .unwrap())
        .collect();

    let max = *heights.iter().max().unwrap();

    let mut smallest_sum = i32::MAX;
    for h in 0..=max {
        let sum: i32 = heights.iter().map(|x| (x-h).abs()).sum();
        if sum < smallest_sum {
            smallest_sum = sum;
        }
    }

    println!("{}", smallest_sum);
}
