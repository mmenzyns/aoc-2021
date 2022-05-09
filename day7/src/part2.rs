use std::fs;

pub fn main() {
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    let heights: Vec<i32> = input.lines().next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let max = *heights.iter().max().unwrap();
    let mut smallest_sum = i32::MAX;

    for h in 0..=max {
        // Calculate sum of binomial coefficients of differences between heights h and crab submarines heights
        let sum: i32 = heights.iter().map(|x| (x-h).abs()).map(|x| x*(x+1)/2).sum();
        
        // Find the smallest sum
        if sum < smallest_sum {
            smallest_sum = sum;
        }
    }

    println!("{}", smallest_sum);
}
