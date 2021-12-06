// Thaddeus "Peter" White | Petras98
// https://adventofcode.com/2021/day/6

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_1() {
    let mut fishes: Vec<i32> = Vec::new();
    const DAYS: i32 = 80;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                fishes = l.split(',').map(|s| s.parse().expect("parse error")).collect();
            }
        }
        
        let mut day = 1;
        let mut num_to_add = 0;

        while day <= DAYS {
            for fish in &mut fishes{
                *fish -= 1;
                if *fish < 0{
                    *fish = 6;
                    num_to_add += 1;
                }
            }

            for _ in 0..num_to_add {
                fishes.push(8);
            }

            num_to_add = 0;
            day += 1;
        }

        println!("{}", fishes.len());

    }

}


fn part_2() {
    //each index corresponds to number of fish at day x
    let mut fishes: [i64; 9] = [0; 9];
    const DAYS: i32 = 256;

    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(l) = line {
                let mut initial: Vec<usize> = Vec::new();
                initial = l.split(',').map(|s| s.parse().expect("parse error")).collect();

                for fish in initial {
                    fishes[fish] += 1;
                }
            }
        }
        
        let mut day = 1;
        let mut num_to_add = 0;

        while day <= DAYS {
            for fish_num in 0..8 {
                if fish_num == 0 {
                    num_to_add = fishes[0];   
                }
                fishes[fish_num] = fishes[fish_num + 1];
            }
            
            fishes[8] = num_to_add;
            fishes[6] += num_to_add;
            day += 1;
        }

        let sum: i64 = fishes.iter().sum();

        println!("{}", sum);

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
