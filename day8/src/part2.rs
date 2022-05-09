use std::fs;

struct Map {
    a: u8,
    b: u8,
    d: u8,
    g: u8,
}

impl Map {
    fn new() -> Map {
        Map {a:0, b:0, d:0, g:0}
    }
}

// Encrypt the digit string into a u8 number, in format "0gfedcba"
fn into_binary(string: &str) -> u8 {
    let mut bin: u8 = 0;
    if string.contains('a') {
        bin |= 0b1;
    }
    if string.contains('b') {
        bin |= 0b10;
    }
    if string.contains('c') {
        bin |= 0b100;
    }
    if string.contains('d') {
        bin |= 0b1000;
    }
    if string.contains('e') {
        bin |= 0b10000;
    }
    if string.contains('f') {
        bin |= 0b100000;
    }
    if string.contains('g') {
        bin |= 0b1000000;
    }
    bin
}

fn recognize_digits(input: Vec<u8>) -> Vec<u8> {
    
    let mut recognized_digits: Vec<u8> = vec![0; 10];

    // Digits ontaining five segments (2,3,5)
    let mut five_segment_digits: Vec<u8> = Vec::with_capacity(3);
    // Digits containing six segments (0,6,9)
    let mut six_segment_digits: Vec<u8> = Vec::with_capacity(3);

    let mut map = Map::new();

    for digit in input {
        let ones = digit.count_ones();
        match ones {
            2 => recognized_digits[1] = digit,
            3 => recognized_digits[7] = digit,
            4 => recognized_digits[4] = digit,
            5 => five_segment_digits.push(digit),
            6 => six_segment_digits.push(digit),
            7 => recognized_digits[8] = digit,
            _ => panic!("match error"),
        }
    }
    assert!(five_segment_digits.len() == 3);
    assert!(six_segment_digits.len() == 3);

    // Subtract 1 digit segments from 7 digit segments, the result is real segment "a"
    map.a = recognized_digits[7] & !recognized_digits[1];

    // If digit (2,3,5) contains both segments from 1, it must be a 3 digit
    for i in 0..five_segment_digits.len() {
        if five_segment_digits[i] & recognized_digits[1] == recognized_digits[1] {
            assert!(recognized_digits[3] == 0);
            recognized_digits[3] = five_segment_digits[i];
            five_segment_digits.remove(i);
            break;
        }
    }

    // Subtract 3 from 4, the resulting segment must be real segment "b"
    map.b = recognized_digits[4] & !recognized_digits[3];

    // If digit (2,5) contains b segment, it must be 5 digit
    for i in 0..five_segment_digits.len() {
        if five_segment_digits[i] & map.b == map.b {
            assert!(recognized_digits[5] == 0);
            recognized_digits[5] = five_segment_digits[i];
            five_segment_digits.remove(i);
            break;
        }
    }

    // Vector contains only one digit, it must be a digit 2
    assert!(five_segment_digits.len() == 1);
    recognized_digits[2] = five_segment_digits[0];

    // Subtract 4 digit and "a" segment from 3 digit to get a "g" digit
    map.g = (recognized_digits[3] & !recognized_digits[4]) - map.a;

    // Subtract 1 digit, "a" segment and "g" segment from 3 digit to get a "d" digit
    map.d = (recognized_digits[3] & !recognized_digits[1]) - map.a - map.g;

    // If digit (0,6,9) doesn't contain a "d" segment, it must be 0 digit
    for i in 0..six_segment_digits.len() {
        if six_segment_digits[i] & map.d != map.d {
            assert!(recognized_digits[0] == 0);
            recognized_digits[0] = six_segment_digits[i];
            six_segment_digits.remove(i);
            break;
        }
    }

    // If digit (6,9) contains segments from digit 1, it must be a 9 digit
    for i in 0..six_segment_digits.len(){
        if six_segment_digits[i] & recognized_digits[1] == recognized_digits[1] {
            assert!(recognized_digits[9] == 0);
            recognized_digits[9] = six_segment_digits[i];
            six_segment_digits.remove(i);
            break;
        }
    }

    // Vector contains only one digit, it must be a 6 digit
    assert!(six_segment_digits.len() == 1);
    recognized_digits[6] = six_segment_digits[0];

    recognized_digits
}

pub fn main() {
    let filename = "input";
    let file = fs::read_to_string(filename).expect("Failed to read file.");

    let mut sum: usize = 0;

    for line in file.lines() {
        let mut sides = line.split('|');
        let input: Vec<u8> = sides
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| into_binary(x))
            .collect();
        let recognized_digits = recognize_digits(input);
        
        let output: Vec<u8> = sides
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| into_binary(x))
            .collect();
        // Map output with recognized digits, to get real digits
        let output_digits: Vec<usize> = output.iter().map(|&x| recognized_digits.iter().position(|&r| r == x).unwrap() as usize).collect();

        // Sum outputs
        sum += output_digits[0] * 1000 + output_digits[1] * 100 + output_digits[2] * 10 + output_digits[3];
    }
    println!("{}",sum);
}
