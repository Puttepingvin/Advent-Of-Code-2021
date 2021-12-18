#![allow(non_snake_case)]

use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Point{
    x : i32,
    y: i32,
    num :i32,
}

fn main() {


    let mut points : Vec<Point> = vec![];
    let lines = parse_input();
    for ix in 0..1000{
        for iy in 0..1000{
            points.push(Point{x: ix, y: iy, num : 0});
        }
    }
    for line in lines{
        let s1 = points.iter_mut().filter(|p| 
            p.x >= cmp::min(line.from_x, line.to_x) 
            && p.y >= cmp::min(line.from_y, line.to_y)  
            && p.x <= cmp::max(line.from_x, line.to_x)  
            && p.y <= cmp::max(line.from_y, line.to_y));
        if line.from_x == line.to_x || line.from_y == line.to_y {
            s1.for_each(|q| q.num += 1);
        }
        else{
            s1.filter(|p|
                (p.y - line.from_y) as f64/ (line.to_y - line.from_y) as f64== (p.x - line.from_x) as f64/ (line.to_x - line.from_x)as f64
            ).for_each(|q| q.num += 1);
        }
    }
/*
    let mut points : Vec<Point> = vec![];
    let lines = parse_input();
    for line in lines{
        for ix in line.from_x..(line.to_x+1){
            for iy in line.from_y..(line.to_y+1){
                let res = points.iter_mut().find(|p| (p.x == ix && p.y == iy));
                match res {
                    Some(p) => p.num += 1,
                    None => points.push(Point{x: ix, y: iy, num : 1})
                }
            }
        }
    }*/
    /*
    let mut points = vec![vec![0;1000];1000];
    let lines = parse_input();
    for line in lines{
        for ix in line.from_x..(line.to_x+1){
            for iy in line.from_y..(line.to_y+1){
                points.filter()+=1;
            }
        }
    }*/

    let tot = points.iter().filter(|p| p.num > 1).count();

    println!("{}", tot)
}

fn dist_between_points_sq(xa : i32, ya : i32, xb : i32, yb : i32) -> i32{
    return (xa-xb)*(xa-xb)+(ya-yb)*(ya-yb);
}

struct Line{
    from_x : i32,
    from_y : i32,
    to_x : i32,
    to_y : i32,
}

fn parse_input() -> Vec::<Line> {
    let mut out = vec![];

    if let Ok(mut lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                let stend : Vec::<&str> = l.split(" -> ").clone().collect();
                let add = Line{
                    from_x: stend[0].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                    from_y: stend[0].split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
                    to_x: stend[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                    to_y: stend[1].split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
                };
                out.push(add);
                
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
