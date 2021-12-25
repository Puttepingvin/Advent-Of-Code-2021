#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Reverse;

#[derive(Clone)]
struct Point{
    x : i32,
    y : i32,
    cost : i32,
    parent_x : i32,
    parent_y : i32,
    forwards : bool
}

//Djikstra backwards-forwards
fn main() {
    let costs_unparsed = parse_input();
    let mut costs : Vec<Vec<i32>>= vec![];
    for j in 0..5{
        for i in 0..costs_unparsed.len() {
            let mut out = vec![];
            for k in 0..5{
                out.extend(costs_unparsed[i].iter().map(|c| (c+j as i32+k as i32 - 1) % 9 + 1))
            }
            costs.push(out);
        }
    }

    let x_sz = costs.len() as i32;
    let y_sz = costs[0].len() as i32;

    let mut explored = vec![vec![false;y_sz as usize];x_sz as usize];
    
    let mut newopen = vec![Point{x : 0, y : 0, cost : 0, parent_x : 0, parent_y : 0, forwards : true},
                           Point{x : x_sz-1, y : y_sz-1, cost : costs[x_sz as usize - 1][y_sz as usize - 1], parent_x : 0, parent_y : 0, forwards : false}];
    let mut closed : Vec<Point> = vec![];
    let mut done = false;
    let mut bestcost = 0;

    while !done{
        let cur = newopen.pop().unwrap(); 
        if cur.cost%100 == 0{
            println!("{},{} - {} at {}", cur.x,cur.y, cur.forwards, cur.cost);
        }
        closed.push(cur.clone());
        for (newx, newy) in [(cur.x - 1, cur.y), (cur.x + 1, cur.y), (cur.x, cur.y - 1), (cur.x, cur.y + 1)]{
            if newx >= 0 && newy >= 0 && newx < x_sz && newy < y_sz {
                let cost = cur.cost + costs[newx as usize][newy as usize];

                if !explored[newx as usize][newy as usize]{
                    let newp = Point{x: newx, y: newy, cost: cost, parent_x : cur.x, parent_y : cur.y, forwards : cur.forwards};
                    explored[newx as usize][newy as usize] = true;
                    match newopen.binary_search_by_key(&Reverse(newp.cost), |p| Reverse(p.cost)) {
                        Ok(pos) => newopen.insert(pos, newp),
                        Err(pos) => newopen.insert(pos, newp),
                    }
                } else if newopen.iter().any(|p| newx == p.x && newy == p.y && p.forwards != cur.forwards) {
                    bestcost = cur.cost + newopen.iter().find(|p| newx == p.x && newy == p.y && p.forwards != cur.forwards).unwrap().cost;
                    done = true;
                    break;
                }

            }
        }
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
