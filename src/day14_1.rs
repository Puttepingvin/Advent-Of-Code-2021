#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {


    let (rules, seed) = parse_input();
    let mut next_gen = seed;
    for _ in 0..40{
        let prev_gen = next_gen.clone();
        next_gen = prev_gen.chars().tuple_windows().map(|(a,b)| {
            match rules.get(&format!("{}{}", a,b)){
                Some(nw) => format!("{}{}", a,nw),
                None => format!("{}", a)
            }
        }).collect::<String>() + &prev_gen.chars().last().unwrap().to_string();
    }   
    let mut counts = vec![];
    for c in next_gen.chars().unique(){
        counts.push(next_gen.chars().filter(|x| x == &c).count());
    } 

    counts.sort();

    println!("{}", counts.iter().last().unwrap()-counts[0])
}

fn parse_input() ->  (HashMap<String, char>, String)  {
    let mut out = HashMap::new();
    let mut seed = "a".to_string();
    if let Ok(mut lines) = read_lines("./input") {
        seed = lines.next().unwrap().unwrap();
        lines.next();
        for row in lines{
            if let Ok(l) = row{
                let mut a = l.split(" -> ");
                out.insert(
                    a.next().unwrap().to_string(),
                    a.next().unwrap().parse().unwrap(),
                );
            }
        }
    }
    return (out, seed);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
