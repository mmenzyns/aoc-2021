use std::fs;


pub fn main() {
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    let mut increases = 0;
    let mut old_depth = u32::MAX;

    for line in input.lines() {
        let depth = match line.trim().parse::<u32>() {
            Ok(depth) => depth,
            Err(_) => continue,
        };

        if depth > old_depth {
            increases += 1;
        }
        old_depth = depth;
    }        
    println!("Depth increases: {}", increases);
}