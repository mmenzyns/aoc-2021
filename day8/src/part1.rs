use std::fs;

pub fn main() {
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    let mut count = 0;
    let counts: Vec<usize> = vec![2, 3, 4, 7];

    for line in input.lines() {
        for digit in line.split('|').last().unwrap().split_whitespace() {
            if counts.contains(&digit.len()) {
                count += 1;
            } 
        }
    }
    println!("{}", count);
}
