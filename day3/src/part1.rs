use std::fs;

fn to_binary(src: Vec<usize>) -> usize {
    let mut number = 0;
    for (i, &digit) in src.iter().rev().enumerate() {
        number |= digit << i;
    }
    number
}

pub fn main() {
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    let nbits = input.lines().clone().next().unwrap().len();
    let mut nlines = 0;

    let mut ones = vec![0;  nbits];

    let mut lsb_lines: Vec<&str> = Vec::new();
    let mut msb_lines: Vec<&str> = Vec::new();

    let mut lines = input.lines().collect();

    for line in input.lines() {
        let mut cnt = 0;
        for i in 0..nbits {
            if line.chars().next().unwrap() == '0' {
                msb_lines.push(line);
            }
        }
        lines = msb_lines;
        println!("{:?}", lines);
        msb_lines = Vec::new();

        for (i, _) in line.chars().enumerate().filter(|(_, c)| *c == '1') {
            ones[i] += 1;
        }
        nlines += 1;
    }
    let gamma_rate: usize = to_binary(ones.iter().map(|&cnt| (cnt > nlines/2) as usize).collect());
    let epsilon_rate: usize = to_binary(ones.iter().map(|&cnt| (cnt < nlines/2) as usize).collect());

    println!("{}, {} = {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}
