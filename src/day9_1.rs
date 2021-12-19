#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {


    let heights = parse_input();

    let mut score = 0;


    

    for i in 0..heights.len(){
        for j in 0..heights[i].len(){
            let mut min = true;
            let this_h = heights[i][j];
            if i != 0{ 
                if this_h >= heights[i-1][j]{
                    min = false;
                }
            }
            if j != 0{ 
                if this_h >= heights[i][j-1]{
                    min = false;
                }
            }
            if i != heights.len() - 1{ 
                if this_h >= heights[i+1][j]{
                    min = false;
                }
            }
            if j != heights[i].len() - 1{ 
                if this_h >= heights[i][j+1]{
                    min = false;
                }
            }
            if min {
                score += this_h + 1;
            }
        }
    }


    println!("{}", score)
}

fn parse_input() ->  Vec::<Vec::<i32>> {
    let mut out : Vec::<Vec::<i32>>= vec![];
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
