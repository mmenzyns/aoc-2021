use std::collections::VecDeque;
use std::fs;


fn summarize(buffer: &VecDeque<u32>) -> u32 {
    let mut sum = 0;
    for i in buffer {
        sum += i;
    }
    sum
}

fn main() {
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    const BUFFER_SIZE: usize = 2;
    let mut increases = 0;
    let mut old_sum = 0;
    let mut buffer: VecDeque<u32> = VecDeque::new();

    for line in input.lines() {
        let depth = match line.trim().parse::<u32>() {
            Ok(depth) => depth,
            Err(_) => continue,
        };
        
        if buffer.len() >= BUFFER_SIZE {
            let sum = summarize(&buffer) + depth;
            if old_sum != 0 && sum > old_sum {
                    increases += 1;
            }
            old_sum = sum;
        }
        buffer.push_front(depth);
        buffer.truncate(BUFFER_SIZE);
    }
    println!("Depth increases: {}", increases);
}
