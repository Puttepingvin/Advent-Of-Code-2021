#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut gamma = 0;
    let mut sigma = 0;
    let mut mult : u32 = 1;

    let input : Vec::<Vec::<u32>> = parse_input();
    
    let num = input[0].len();
    for i in 0..num{
        let mut balance = 0;
        for vc in &input{
            let x = vc[num-i-1];
            if x == 0{
                 balance-=1;
            }
            if x == 1{
                balance+=1;
            }
        }
        if balance > 0{
             gamma += mult;
        }
        else{
            sigma += mult;
        }
        mult *= 2;
        println!("{:b}, {}, {:b}, {}", gamma, gamma, sigma, sigma);
    }


    println!("{}", sigma*gamma)
}

fn parse_input() -> Vec::<Vec::<u32>> {
    let mut out = vec![];
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                out.push(ip.split("").flat_map(|x| x.parse::<u32 >()).collect());
                //if let Ok(intg) = {
                //}
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
