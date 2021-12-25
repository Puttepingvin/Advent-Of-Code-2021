#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Way{
    points : Vec<Point>,
    cost : i32
} 
#[derive(Clone)]
struct Point{
    x: i32,
    y : i32
}
impl Point {

}
//Djikstra
fn main() {
    let costs = parse_input();
    let x_sz = costs.len() as i32;
    let y_sz = costs[0].len() as i32;
    let mut bests = vec![vec![2147483647;costs[0].len()];costs.len()];

    let mut paths = vec![Way{points : vec![Point{x: 0, y : 0}], cost : 0}];
    let mut done = false;
    let mut bestcost = 0;
    let mut bestpath = Way{points : vec![Point{x: 0, y : 0}], cost : 0};
    while !done{
        let mut paths_old = paths.clone();
        let mut newbests = bests.clone();
        //paths.sort_by(|p| p.cost);
        let (i, cur) = paths_old.iter_mut().enumerate().min_by_key(|(_, p)| p.cost).unwrap();
        let p = cur.points.iter().last().unwrap();
        println!("{},{}", p.x,p.y);
        let poss_unfiltered = vec![(p.x -1, p.y), (p.x, p.y - 1), (p.x+1, p.y), (p.x, p.y+1)];
        let possibles : Vec::<&(i32, i32)> = poss_unfiltered.iter()
                    .filter(|(x,y)| x >= &0  && y >= &0 && *x < x_sz && *y < y_sz)
                    .filter(|(x,y)| !cur.points.iter().any(|pt| &pt.x == x && &pt.y == y)).collect();
        paths.swap_remove(i);
        if possibles.len() != 0{
            for (x,y) in possibles.iter(){
                let mut newp = cur.clone();
                newp.cost += costs[*x as usize][*y as usize];
                newp.points.push(Point{x : *x, y: *y});
                if *x == x_sz-1 && *y == y_sz-1{
                    done = true;
                    bestcost = newp.cost;
                    bestpath = newp.clone();
                }
                if bests[*x as usize][*y as usize] > newp.cost{
                    newbests[*x as usize][*y as usize] = newp.cost.clone();
                    paths.push(newp);
                }
            }
        }
        bests = newbests;
    }

    for p in bestpath.points.iter(){
        println!("{}, {}", p.x, p.y)
    }



    println!("{}", bestcost)
}

fn parse_input() ->  Vec<Vec<i32>> {
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
