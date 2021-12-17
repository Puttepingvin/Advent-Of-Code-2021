#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;

    let input = parse_input();

    for inp in input{
        let ar : Vec::<&str>= inp.split(" ").collect();
        let command = ar[0];
        let num = ar[1].parse::<i32>().unwrap();   
        match command{
            "forward" => {
                x +=num;
                z += num*aim;
            },
            "down" => aim +=num,
            "up" => aim -=num,
            _ => (),
        }

    }


    println!("{}", x*z)
}

fn parse_input() -> Vec::<String> {
    let mut out = vec![];
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                out.push(ip)
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
