#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;


fn main() {


    let mut energy = parse_input();
    let mut flashes = 0;
    let mut stepnum = 0;
    for _ in 0..100{
        stepnum+=1;
        for i in 0..10{
            for j in 0..10{
                energy[i][j] += 1;
            }
        }
        let mut popped = vec![vec![false;10];10];

        while (0..10).any(|i| (0..10).any(|j| popped[i][j] == false && energy[i][j] >= 10)) {
            for i in 0..10{
                for j in 0..10 {
                    if popped[i][j] == false && energy[i][j] >= 10 {
                        popped[i][j] = true;
                        flashes += 1;
                        for k in cmp::max(i as i32-1,0)..cmp::min(i as i32+2,10){
                            for l in cmp::max(j as i32-1,0)..cmp::min(j as i32+2,10){
                                energy[k as usize][l as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
        for e in energy.iter_mut().flatten().filter(|x| **x >= 10){
            *e = 0;
        }
    }
        
    
    println!("{}", flashes)
}

fn parse_input() ->  Vec::<Vec::<i32>> {
    let mut out = vec![];
    if let Ok(lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                out.push(l.trim().split("").flat_map(|x| x.parse::<i32>()).collect());
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
