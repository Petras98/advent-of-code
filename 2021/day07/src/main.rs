// Thaddeus "Peter" White | Petras98
// https://adventofcode.com/2021/day/5

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//how many bits


fn part_1() {
    let mut crabs: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                crabs = l.split(',').map(|s| s.parse().expect("parse error")).collect();
            }
        }
    }

    let max = crabs.iter().max().unwrap();

    let mut min_fuel = -1;
    let mut match_num = -1;

    for num in 0..max + 1 {
        let mut fuel = 0;
        for crab in &mut crabs {
            fuel += {if num > *crab {num - *crab} else {*crab - num} }
        }

        if min_fuel == -1 {
            min_fuel = fuel;
            match_num = num;
            continue;
        }

        if fuel < min_fuel{
            min_fuel = fuel;
            match_num = num;
        }
    }

    println!("fuel consumption {}", min_fuel);
}

fn part_2() {
    let mut crabs: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                crabs = l.split(',').map(|s| s.parse().expect("parse error")).collect();
            }
        }
    }

    let max = crabs.iter().max().unwrap();

    let mut min_fuel = -1;
    let mut match_num = -1;

    for num in 0..max + 1 {
        let mut fuel = 0;
        for crab in &mut crabs {
            fuel += {if num > *crab {
                let diff = (num - *crab);
                (diff * (diff+1))/2
            } else {
                
                let diff = *crab - num;
                (diff * (diff+1))/2
            } }
        }

        if min_fuel == -1 {
            min_fuel = fuel;
            match_num = num;
            continue;
        }

        if fuel < min_fuel{
            min_fuel = fuel;
            match_num = num;
        }
    }

    println!("fuel consumption {}", min_fuel);
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
