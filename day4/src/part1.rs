use std::fs;

#[derive(Clone)]
struct Board {
    numbers: Vec<Vec<i16>>,
    marked: Vec<Vec<bool>>,
}

fn parse_boards(mut line_iter: std::str::Lines) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    while line_iter.next() != None {
        let mut rows: Vec<Vec<i16>> = Vec::new();
        for _ in 0..5 {
            let line = line_iter.next();
            if line == None {
                break;
            }
            let row: Vec<i16> = line.unwrap().split_whitespace().map(|x| x.parse::<i16>().unwrap()).collect();
            rows.push(row);
        }
        let board = Board {
            numbers: rows,
            marked: vec![vec![false; 5]; 5],
        };
        boards.push(board);
    }
    return boards;
}

fn get_winning_board(mut boards: Vec<Board>, picks: Vec<i16>) -> (Board, i16) {
    for pick in picks {
        // For each pick number, go through all boards and mark this number
        for board in boards.iter_mut() {
            for (irow, row) in board.numbers.iter().enumerate() {
                for (icol, &number) in row.iter().enumerate() {
                    if number == pick {
                        board.marked[irow][icol] = true;
                    }
                }
            }

            // Count marked numbers in bingo patterns
            let mut col_sums = vec![0; 5];
            for row in board.marked.iter() {
                let mut row_sum = 0;
                for (icol, &col) in row.iter().enumerate() {
                    if col == true {
                        row_sum += 1; // Count marked in a row
                        col_sums[icol] += 1; // Count marked in a col
                    }
                }
                if row_sum >= 5 {
                    return (board.clone(), pick);
                }
            }

            for col_sum in col_sums {
                if col_sum >= 5 {
                    return (board.clone(), pick);
                }
            }
        }
    }
    panic!("No bingo");
}

pub fn main() {
    // Read file
    let filename = "example";
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    // Extract picked numbers and playing boards
    let mut line_iter = input.lines();
    let picks: Vec<i16> = line_iter.next().unwrap().split(',').map(|x| x.parse::<i16>().unwrap()).collect();
    let boards = parse_boards(line_iter);

    // Find winning board
    let (winning_board, winning_pick) = get_winning_board(boards, picks);

    // Calculate the result (Sum of unmarked numbers times winning pick)
    let mut sum: u32 = 0;
    for (irow, row) in winning_board.marked.iter().enumerate() {
        for (icol, &marked) in row.iter().enumerate() {
            if !marked {
                sum += winning_board.numbers[irow][icol] as u32;
            }
        }
    }
    println!("{} x {} = {}", sum, winning_pick, sum * winning_pick as u32);
}
