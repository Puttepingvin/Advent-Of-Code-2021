#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {


    let (max, mut pos) = parse_input();
    let mut cur_min = 0;
    let mut cur_max = max-1;
    let mut fuel = 0;
    while cur_min != cur_max{
        if pos[cur_min] < pos[cur_max]{
            pos[cur_min+1] += pos[cur_min];
            fuel += pos[cur_min];
            cur_min += 1;
        }
        else{
            pos[cur_max-1] += pos[cur_max];
            fuel += pos[cur_max];
            cur_max -= 1;
        }
    }

    println!("{}, {}", fuel, cur_max)
}

fn parse_input() -> (usize, Vec::<usize>) {
    let mut out = vec![];
    let mut nums = 0;
    if let Ok(lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                nums = l.split(",").map(|x| x.parse::<usize>().unwrap()).max().unwrap()+1;
                out = vec![0; nums];
                for i in 0..nums{
                    out[i] = l.split(",").filter(|y| y.parse::<usize>().unwrap() == i).count();
                }                                
            }
        }
    }
    return (nums, out);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
