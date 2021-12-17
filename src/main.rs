#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut gamma = 0;
    let mut sigma = 0;

    let mut input_big : Vec::<Vec::<u32>> = parse_input();
    let mut input_small : Vec::<Vec::<u32>> = parse_input();
    
    let num :usize = input_big[0].len();
    for i in 0..num {
        let mut balance = input_big.iter().map(|x| x[i]).sum::<u32>();

        if balance as f32 >= (input_big.len() as f32)/(2 as f32) {
            input_big = input_big.into_iter().filter(|x| x[i] == 1).collect();
        }
        else{
            input_big = input_big.into_iter().filter(|x| x[i] == 0).collect();
        }

        balance = input_small.iter().map(|x| x[i]).sum::<u32>();
        if balance as f32 >= (input_small.len() as f32)/(2 as f32) {
            if balance != (input_small.len() as u32){
                input_small = input_small.into_iter().filter(|x| x[i] == 0).collect();
            }
        }
        else{
            if balance != 0{
                input_small = input_small.into_iter().filter(|x| x[i] == 1).collect();
            }
        }
    }
    for (i, v) in input_big[0].iter().enumerate(){
        gamma += v*(1<<(num-i-1));
    }
    for (i, v) in input_small[0].iter().enumerate(){
        sigma += v*(1<<(num-i-1));
    }


    println!("{:b}, {:b}, {}", gamma, sigma, gamma*sigma)
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
