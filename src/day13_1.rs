#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Point{
    x : i32,
    y : i32,
}

struct Instr{
    dir : char,
    place : i32,
}

fn main() {


    let (mut points, instrs) = parse_input();

    for instr in instrs.iter(){
        for point in points.iter_mut(){
            if instr.dir == 'x'{
                if point.x > instr.place{
                    point.x = instr.place - (point.x - instr.place);
                }
            }
            else if instr.dir == 'y'{
                if point.y > instr.place{
                    point.y = instr.place - (point.y - instr.place);
                }
            }
        }
        break;
    }
    let mut count = 0;
    let mut image = vec![vec!['.';(points.iter().max_by_key(|p| p.x).unwrap().x + 1) as usize];(points.iter().max_by_key(|p| p.y).unwrap().y + 1) as usize];
    for p in points.iter(){
        if image[p.y as usize][p.x as usize] == '.'{
            image[p.y as usize][p.x as usize] = '#';
            count += 1;
        }
    }
    for line in image.iter(){
        println!("{}", line.iter().collect::<String>())
    }

    println!("{}", count)
}

fn parse_input() ->  (Vec<Point>, Vec<Instr>)  {
    let mut out : Vec<Point>  = vec![];
    let mut instr : Vec<Instr>  = vec![];
    if let Ok(lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                let nums : Vec<&str> = l.split(",").collect();
                if nums.len() == 2{
                    out.push(Point{
                        x : nums[0].parse().unwrap(),
                        y : nums[1].parse().unwrap()
                    })
                }
                else {
                    let instrs : Vec<&str> = l.split("=").collect();
                    if instrs.len() == 2{
                        instr.push(Instr{
                            dir : instrs[0].chars().last().unwrap(),
                            place : instrs[1].parse().unwrap()
                        })
                    }
                }
            }
        }
    }
    return (out, instr);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
