// Thaddeus "Peter" White | Petras98
// https://adventofcode.com/2021/day/3

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//how many bits
const len: usize = 12;

fn part_1() {
    let mut bytes: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(byte) = line {
                
                let intval = isize::from_str_radix(&byte, 2).unwrap();
                bytes.push(intval.try_into().unwrap());
            }
        }
    }

    let mut ones: [i32; len] = [0; len];
    let mut zeros: [i32; len] = [0; len];

    for byte in bytes{
        for index in (0..len).rev() {
            let current_bit = (byte >> index) & 1;
            if current_bit == 0 {
                zeros[index] += 1;
            }else {
                ones[index] += 1;
            }
        }
    }


    let mut gamma = 0;
    let mut epis = 0;

    //form the final numbers 
    for index in (0..len).rev(){
        if ones[index] > zeros[index]{
            gamma |= (1 << index);
            epis |= (0 << index);
        }else{
            gamma |= (0 << index);
            epis |= (1 << index);
        }
    }


    println!("part_1 gamma * epis: {}", gamma * epis);
}


fn part_2_helper(mut bytes: Vec<i32>, find_gamma: bool) -> i32 {
    //counts of ones and zeros at a specific index
    let mut ones: [i32; len] = [0; len];
    let mut zeros: [i32; len] = [0; len];

    //save the numbers that start with 1s and 0s
    let mut ones_bytes: Vec<i32> = Vec::new();
    let mut zeros_bytes: Vec<i32> = Vec::new();

    //return value
    let mut value: i32 = 0;

    //loop over, start at the last index
    for index in (0..len).rev() { 

        for byte in bytes {
            let current_bit = (byte >> index) & 1;
            if current_bit == 0 {
                zeros[index] += 1;
                zeros_bytes.push(byte);

            }else {
                ones[index] += 1;
                ones_bytes.push(byte);
            }
        }

        //check if there were more 1s than zeros
        if ones[index] >= zeros[index]{
            //set the bytes vector to either just the numbers starting with ones or zeros
            bytes = if find_gamma {ones_bytes.clone()} else {zeros_bytes.clone()};
        }else{ 
            bytes = if find_gamma {zeros_bytes.clone()} else {ones_bytes.clone()};
        }

        ones_bytes.clear();
        zeros_bytes.clear();

        //if only one value is left
        if bytes.len() == 1 {
            value = bytes[0];
            break;
        }
    }
    value
}

fn part_2() {
    //the full bytes list
    let mut bytes_full: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(byte) = line {
                
                //convert binary to isize
                let intval = isize::from_str_radix(&byte, 2).unwrap();
                //push an i32
                bytes_full.push(intval.try_into().unwrap());
            }
        }
    }

    //deep clone bytes
    let mut bytes: Vec<i32> = bytes_full.clone();

    //get gamma
    let gamma = part_2_helper(bytes, true);
    //reset bytes
    bytes = bytes_full.clone();

    //get epsilon
    let epis =  part_2_helper(bytes, false);

    println!("part_2 gamma * epis: {}", gamma * epis);
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
