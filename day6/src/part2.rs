use std::fs;

pub fn main() {
    let filename = "input";
    let end_time = 256;
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    // Parse input into Vector of Lanternfish structs
    let initial_fishes: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut fish_counter: usize = initial_fishes.len(); // Fishes counter, initiate with initial fishes already counted
    let mut spawn_calendar: Vec<usize> = vec![0; end_time + 9]; // Index represents each day and value how many fishes will spawn that day

    for initial_fish in initial_fishes {
        spawn_calendar[initial_fish] += 1; // Add spawn events of each initial fish
    }

    for time in 0..end_time {
        let spawns = spawn_calendar[time];
        if spawns != 0 {
            spawn_calendar[time + 7] += spawns; // 7 days from now, fishes will reproduce again
            spawn_calendar[time + 9] += spawns; // 9 days from now, newly born fishes will reach maturity and reproduce
            fish_counter += spawns; // Add newly born fishes into the fish counter
        }
    }
    println!("{}", fish_counter);
}
