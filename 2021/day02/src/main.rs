// Thaddeus "Peter" White | Petras98
// https://adventofcode.com/2021/day/2

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_1() {
    //[horizontal, depth]
    let mut pos: [i32; 2] = [0; 2];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(full_command) = line {
                //split by whitespace
                let split: Vec<&str> = full_command.split_whitespace().collect();
                //forward/up/down
                let command_direction = split[0];
                //X units
                let distance = split[1];

                //convert to number
                let num: i32 = distance.parse().unwrap();

                if command_direction.eq("forward") {
                    pos[0] += num;
                }else if command_direction.eq("up"){
                    pos[1] -= num;
                }else if command_direction.eq("down"){
                    pos[1] += num;
                }
            }
        }
    }

    println!("part_1 x * y: {}", pos[0] * pos[1]);
}


fn part_2() {
    //[horizontal, depth, aim]
    let mut pos: [i128; 3] = [0; 3];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(full_command) = line {
                let split: Vec<&str> = full_command.split_whitespace().collect();
                let command_direction = split[0];
                let distance = split[1];

                //convert to number
                let num: i128 = distance.parse().unwrap();

                if command_direction.eq("forward") {
                    pos[0] += num;
                    //increase depth
                    pos[1] += num * pos[2];
                    
                }else if command_direction.eq("up"){
                    pos[2] -= num;
                }else if command_direction.eq("down"){
                    pos[2] += num;
                }
            }
        }
    }

    println!("part_2 x * y: {}", pos[0] * pos[1]);
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
