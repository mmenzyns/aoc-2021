use std::fs;

struct Lanternfish {
    timer: usize,
}

impl Lanternfish {
    fn new(timer: usize) -> Lanternfish {
        Lanternfish { timer: timer }
    }
}

trait Advance {
    fn advance_day(&mut self);
}

impl Advance for Vec<Lanternfish> {
    fn advance_day(&mut self) {
        for i in 0..self.len() {
            if self[i].timer == 0 {
                // Fish gave a birth, reset it's counter to 6, and create new fish with counter set to 8
                self[i].timer = 6;
                self.push(Lanternfish::new(8));
            }
            else {
                self[i].timer -= 1;
            }  
        }
    }
}

pub fn main() {
    let filename = "example";
    let end_time = 80;
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    let mut fishes: Vec<Lanternfish> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| Lanternfish::new(x
            .parse::<usize>()
            .unwrap()))
        .collect();

    for _ in 0..end_time {
        fishes.advance_day();
    }

    println!("{}", fishes.len());
}
