// Thaddeus "Peter" White | Petras98
// https://adventofcode.com/2021/day/1

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_1() {
    let mut num_increases = -1;
    let mut prev_depth: i32 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                //convert to number
                let depth: i32 = num.parse().unwrap();
                
                //if it's the first num
                if num_increases == -1{
                    prev_depth = depth;
                    num_increases = 0;

                    continue;
                }
                //check if the current depth is greater than the last
                if depth > prev_depth {   
                    num_increases += 1;
                }
                prev_depth = depth;
            }
        }
    }

    println!("num_increase part_1: {}", num_increases);
}


fn part_2() {
    let mut num_increases = -1;
    let mut index = 0;
    let mut prev_sum: i32 = 0;
    let mut depths: [i32; 3] = [0; 3];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                let depth: i32 = num.parse().unwrap();
                
                //if it's the first num
                if num_increases == -1{
                    depths[index] = depth;
                    index += 1;
                    
                    if index == 3 {
                        prev_sum = depths.iter().sum();
                        num_increases = 0;
                        index = 0;
                        println!("{}", prev_sum);
                    }
                    continue;
                }

                //shift the depths down 1 and add the current depth
                depths[0] = depths[1];
                depths[1] = depths[2];
                depths[2] = depth;

                let sum = depths.iter().sum();

                if sum > prev_sum {
                    num_increases += 1;
                }
                prev_sum = sum;
            }
        }
    }

    println!("num_increase part_2: {}", num_increases);
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
