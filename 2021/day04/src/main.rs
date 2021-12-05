// Thaddeus "Peter" White | Petras98
// https://adventofcode.com/2021/day/4

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const board_dim: (usize, usize) = (5, 5);

struct Board {
    nums: [[(i32, bool); board_dim.1]; board_dim.0],
    //plus two for two diagonals
    matched_totals: [i32; board_dim.0 + 2],
    matched_totals_col: [i32; board_dim.1 + 2],
    is_solved: bool,
    final_num: i32,
}

fn part_1() {
    let mut drawn: Vec<i32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut curr_board = 0;
    let mut curr_row = 0;


    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                //if we don't have the drawn numbers yet
                if drawn.len() == 0 {
                    //split by commas, then convert to numbers
                    drawn = l.split(',')
                    .map(|s| s.parse().expect("parse error")).collect();
                    continue;
                }
                
                if l.trim().len() == 0 {
                    boards.push(Board{
                        nums: [[(0, false); board_dim.1]; board_dim.0],
                        matched_totals: [0; board_dim.0 + 2],
                        matched_totals_col: [0; board_dim.0 + 2],
                        is_solved: false,
                        final_num: 0,
                    });
                    curr_board = boards.len() - 1;
                    curr_row = 0;

                    continue;
                }else{
                    let row = l.split_whitespace().map(|s| s.parse().expect("parse error"));
                    
                    let mut index = 0;
                    for num in row {
                        boards[curr_board].nums[curr_row][index].0 = num;
                        index += 1;
                    }

                    curr_row += 1;
                }
                
                
            }
        }

        let mut final_board_index = 0;
        let mut final_num = 0;

        'top: for draw in drawn {
            for board_index in 0..boards.len(){
                for row_index in 0..board_dim.0 {
                    for col_index in 0..board_dim.1 {
                        if draw == boards[board_index].nums[row_index][col_index].0 {
                            boards[board_index].nums[row_index][col_index].1 = true;
                            
                            boards[board_index].matched_totals[row_index] += 1;
                            boards[board_index].matched_totals_col[col_index] += 1;

                            if boards[board_index].matched_totals[row_index] == 5 || boards[board_index].matched_totals_col[col_index] == 5{
                                final_board_index = board_index;
                                final_num = boards[board_index].nums[row_index][col_index].0;
                                break 'top;
                            }
                        }
                    }
                }
            }
        }

        println!("Final board: ");

        let mut unmatched_sum = 0;
        for row in boards[final_board_index].nums{
            for col in row{
                if !col.1 {
                    unmatched_sum += col.0;
                }
                print!("{} ", col.0);
            }
            println!();
        }
        println!("part_1 unmatched * last_num: {}", unmatched_sum*final_num);
    }

    
}


fn part_2() {
    let mut drawn: Vec<i32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut solved_boards_indexes: Vec<usize> = Vec::new();
    let mut curr_board = 0;
    let mut curr_row = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                //if we don't have the drawn numbers yet
                if drawn.len() == 0 {
                    //split by commas, then convert to numbers
                    drawn = l.split(',')
                    .map(|s| s.parse().expect("parse error")).collect();
                    continue;
                }
                
                if l.trim().len() == 0 {
                    boards.push(Board{
                        nums: [[(0, false); board_dim.1]; board_dim.0],
                        matched_totals: [0; board_dim.0 + 2],
                        matched_totals_col: [0; board_dim.0 + 2],
                        is_solved: false,
                        final_num: 0,
                    });
                    curr_board = boards.len() - 1;
                    curr_row = 0;

                    continue;
                }else{
                    let row = l.split_whitespace().map(|s| s.parse().expect("parse error"));
                    
                    let mut index = 0;
                    for num in row {
                        boards[curr_board].nums[curr_row][index].0 = num;
                        index += 1;
                    }

                    curr_row += 1;
                }
                
                
            }
        }

        

        for draw in drawn {
            for board_index in 0..boards.len(){
                if boards[board_index].is_solved {continue};
                for row_index in 0..board_dim.0 {
                    for col_index in 0..board_dim.1{
                        if draw == boards[board_index].nums[row_index][col_index].0 {
                            boards[board_index].nums[row_index][col_index].1 = true;
                            
                            boards[board_index].matched_totals[row_index] += 1;
                            boards[board_index].matched_totals_col[col_index] += 1;

                            if boards[board_index].matched_totals[row_index] == 5 || boards[board_index].matched_totals_col[col_index] == 5{
                                boards[board_index].final_num = boards[board_index].nums[row_index][col_index].0;
                                boards[board_index].is_solved = true;
                                solved_boards_indexes.push(board_index);
                            }
                        }
                    }
                }
            }
        }

        println!("Final board: ");
        
        let final_board_index = *solved_boards_indexes.last().unwrap();

        let mut unmatched_sum = 0;
        for row in boards[final_board_index].nums{
            for col in row{
                if !col.1{
                    unmatched_sum += col.0;
                }
                print!("{} ", col.0);
            }
            println!();
        }
        println!("part_2 unmatched * last_num: {}", unmatched_sum*boards[final_board_index].final_num);
    }

    
}

fn main() {
    part_1();
    part_2();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
