use std::cmp;
use std::fs;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn swap_points(&mut self) {
        let temp = self.p1;
        self.p1 = self.p2;
        self.p2 = temp;
    }
}

fn parse_input(input: String) -> Vec<Line> {
    // Extract picked numbers and playing boards
    let mut lines: Vec<Line> = Vec::new();

    for strline in input.lines() {
        if strline.is_empty() {
            break;
        }

        // Parse the file
        let points: Vec<&str> = strline.split(" -> ").collect();

        assert_eq!(points.len(), 2);
        let point1: Vec<usize> = points[0]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        assert_eq!(point1.len(), 2);

        let point2: Vec<usize> = points[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        assert_eq!(point2.len(), 2);

        // Create a Line structure and push into a vector
        let line = Line {
            p1: Point {
                x: point1[0],
                y: point1[1],
            },
            p2: Point {
                x: point2[0],
                y: point2[1],
            },
        };

        lines.push(line);
    }
    lines
}

fn count_overlaps(mut lines: Vec<Line>) -> Vec<Vec<i32>> {
    // Create a 2D vector for counting overlaps with size determined by max values in coordinatess    
    let xsize = lines.iter().map(|x| cmp::max(x.p1.x, x.p2.x)).max().unwrap().clone() + 1;
    let ysize = lines.iter().map(|x| cmp::max(x.p1.y, x.p2.y)).max().unwrap().clone() + 1;
    let mut overlaps = vec![vec![0; ysize]; xsize];

    // Count overlaps
    for line in lines.iter_mut() {
        if line.p1.x > line.p2.x {
            // Swap, so a for range can be created easily
            line.swap_points();
        }

        if line.p1.x == line.p2.x {
            // Vertical
            let x = line.p1.x;
            let ymin = cmp::min(line.p1.y, line.p2.y);
            let ymax = cmp::max(line.p1.y, line.p2.y);

            for y in ymin ..= ymax {
                overlaps[x][y] += 1;
            }
        } else {
            // Horizontal or diagonal
            let ystep = (line.p2.y as i32 - line.p1.y as i32).signum();
            let mut y = line.p1.y as i32;

            for x in line.p1.x ..= line.p2.x {
                assert!(y >= 0);
                overlaps[x][y as usize] += 1;
                y += ystep;
            }
        }
    }
    overlaps
}

pub fn main() {
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");
    let lines = parse_input(input);
    let overlaps = count_overlaps(lines);
    
    // Count at how many points do at least two lines overlap
    let mut result = 0;
    for row in overlaps {
        result += row.iter().filter(|&x| *x >= 2).count();
    }
    println!("{}", result);
}
