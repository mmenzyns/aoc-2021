use std::fs;

struct Position {
    horiz: i32,
    depth: i32,
}

pub fn main() {
    let mut pos = Position {
        horiz: 0,
        depth: 0,
    };

    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();

        let command = tokens[0];
        let length: i32 = tokens[1].parse().unwrap();
        match command {
            "forward" => pos.horiz += length,
            "up" => pos.depth -= length,
            "down" => pos.depth += length,
            _ => panic!("Submarine command {} not recognized.", command),
        }
    }
    println!("Horizontal position times depth: {}", pos.horiz * pos.depth);
}
