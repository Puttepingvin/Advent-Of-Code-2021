#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {


    let heights = parse_input();

    let w = heights.len();
    let h = heights[0].len();

    let mut basins = vec![vec![0;h];w];

    //let mut score = 0;
    let mut basin_id = 1;


    

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
                basins[i][j] = basin_id;
                basin_id += 1;
            }
        }
    }

    let mut basinsizes = vec![1;basin_id];
    let mut done = false;
    while !done{
        done = true;    
        for i in 0..heights.len(){
            for j in 0..heights[i].len(){
                if basins[i][j] == 0 && heights[i][j] != 9{
                    let mut foundbasin = 0;
                    if i != 0{ 
                        if basins[i-1][j] > 0  {
                            foundbasin = basins[i-1][j];
                        }
                    }
                    if j != 0{ 
                        if basins[i][j-1] > 0 {
                            foundbasin = basins[i][j-1];
                        }
                    }
                    if i != heights.len() - 1{ 
                        if basins[i+1][j] > 0 {
                            foundbasin = basins[i+1][j];
                        }
                    }
                    if j != heights[i].len() - 1 { 
                        if basins[i][j+1] > 0{
                            foundbasin = basins[i][j+1];
                        }
                    }
                    if foundbasin != 0 {
                        basins[i][j] = foundbasin;
                        basinsizes[basins[i][j]] += 1;
                        done = false;    
                    }
                }
            }
        }
    }
    basinsizes.sort();
    basinsizes.reverse();
    println!("{}", basinsizes[0]*basinsizes[1]*basinsizes[2])
}

fn parse_input() ->  Vec::<Vec::<i32>> {
    let mut out : Vec::<Vec::<i32>>= vec![];
    if let Ok(lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                out.push(l.trim().split("").flat_map(|x| x.parse::<i32>()).collect());
                //out.last().unwrap().append(10);
                //out.last().unwrap().prepend(10);
            }
        }
    }
    //out.push(out[0].iter().flat_map(|x| *x+1).collect());
    //out.unshift(out[0].iter().flat_map(|x| *x+1).collect());
    return out;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
