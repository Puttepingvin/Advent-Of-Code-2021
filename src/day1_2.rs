#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut n = -1;
    let mut last = -1;
    let input = parse_input();

    for i in 0..(input.len()-2){
        let x = input[i] + input[i+1] + input[i+2];
        if x > last{
            n+=1;
        }
        last = x;
    }


    println!("{}", n)
}

fn parse_input() -> Vec::<i32> {
    let mut out = vec![];
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Ok(i) = ip.parse::<i32>() {
                    out.push(i)
                }
            }
        }
    }
    return out;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
