#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {


    let mut cur_gen = parse_input();
    for i in 0..80{
        let mut next_gen = vec![0;9];
        for i in 1..9{
            next_gen[i-1] = cur_gen[i];
        }
        next_gen[8] = cur_gen[0];
        next_gen[6] += cur_gen[0];
        cur_gen = next_gen;
    }

    println!("{}", cur_gen.iter().sum::<usize>())
}

fn parse_input() -> Vec::<usize> {
    let mut out = vec![0;9];

    if let Ok(mut lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                for i in 0..9{
                    out[i] = l.split(",").filter(|x| x.parse::<usize>().unwrap() == i).count();
                    println!("{}", out[i])
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
