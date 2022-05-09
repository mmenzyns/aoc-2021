use std::fs;
use std::collections::VecDeque;
use itertools::iproduct;

#[derive(Clone, Debug)]
struct Board {
    numbers: Vec<Vec<i16>>,
    marked: Vec<Vec<bool>>,
}

fn parse_boards(mut line_iter: std::str::Lines) -> VecDeque<Board> {
    let mut boards: VecDeque<Board> = VecDeque::new();
    // Vec<Board> = Vec::new();
    while line_iter.next() != None {
        let mut rows: Vec<Vec<i16>> = Vec::new();
        for _ in 0..5 {
            let line = line_iter.next();
            if line == None {
                break;
            }
            let row: Vec<i16> = line
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i16>().unwrap())
                .collect();
            rows.push(row);
        }
        if rows.len() != 5 {
            break;
        }
        let board = Board {
            numbers: rows,
            marked: vec![vec![false; 5]; 5],
        };
        boards.push_back(board);
    }
    boards
}

fn get_last_winning_board(mut boards: VecDeque<Board>, picks: Vec<i16>) -> (Board, i16) {
    for (&pick, _) in iproduct!(picks.iter(), 0..boards.len()) {
        if let Some(mut board) = boards.pop_front() {
            // let mut board = board; // LLDB breaks so this makes me see the variable in debugger
            for (irow, row) in board.numbers.iter().enumerate() {
                for (icol, _) in row.iter().enumerate().filter(|(_, &number)| number == pick) {
                    board.marked[irow][icol] = true;
                }
            }
            // Count marked numbers in bingo patterns
            let mut bingo = false;
            let mut col_sums = vec![0; 5];
            for row in board.marked.iter() {
                let mut row_sum = 0;
                for (icol, _) in row.iter().enumerate().filter(|(_, &col)| col == true) {
                    row_sum += 1; // Count marked in a row
                    col_sums[icol] += 1; // Count marked in a col
                }
                if row_sum >= 5 {
                    bingo = true;
                }
            }
            for _ in col_sums.iter().filter(|&&col_sum| col_sum >= 5) {
                bingo = true;
            }

            if bingo && boards.len() == 0 {
                return (board, pick);
            }

            if !bingo {
                boards.push_back(board);
            }
        }
    }
    panic!("No bingo");
}

pub fn main() {
    // Read file
    let filename = "input";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    // Extract picked numbers and playing boards
    let mut line_iter = input.lines();
    let picks: Vec<i16> = line_iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i16>().unwrap())
        .collect();
    let boards = parse_boards(line_iter);

    // Find winning board
    let (winning_board, winning_pick) = get_last_winning_board(boards, picks);

    // Calculate the result (Sum of unmarked numbers times winning pick)
    let mut sum: u32 = 0;
    for (irow, row) in winning_board.marked.iter().enumerate() {
        for (icol, _) in row.iter().enumerate().filter(|(_, &marked)| !marked) {
            sum += winning_board.numbers[irow][icol] as u32;
        }
    }
    println!("{} x {} = {}", sum, winning_pick, sum * winning_pick as u32);
}
