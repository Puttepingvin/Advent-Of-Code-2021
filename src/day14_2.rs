#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {


    let (rules, seed) = parse_input();
    let mut counts_blank : HashMap::<&String, i64> = HashMap::new();

    for k in rules.keys(){
        counts_blank.insert(k, 0);
    }

    let mut counts = counts_blank.clone();

    for (a,b) in seed.chars().tuple_windows(){
        if let Some(c) = counts.get_mut(&format!("{}{}", a,b)) {
            *c += 1;
        } 
    }

    for _ in 0..40{
        let mut counts_new = counts_blank.clone();
        for (k,v) in rules.iter(){
            if let Some(curr) = counts.get(k) {
                if let Some(c1) = counts_new.get_mut(&format!("{}{}", k.chars().next().unwrap(), v)){
                    *c1 += curr;
                }
                if let Some(c2) = counts_new.get_mut(&format!("{}{}", v,k.chars().last().unwrap())){
                    *c2 += curr;
                }
            } 
        }
        counts = counts_new.clone();
    }   
    
    let mut counts_single = HashMap::new();

    for k in counts.keys(){
        *counts_single.entry(k.chars().next().unwrap()).or_insert(0) += counts.get(k).unwrap();
    }
    *counts_single.entry(seed.chars().last().unwrap()).or_insert(0) += 1;

    println!("{}", counts_single.values().max().unwrap() - counts_single.values().min().unwrap())
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
