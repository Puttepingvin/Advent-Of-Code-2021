#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {


    let nodes = parse_input();

    let mut paths = vec![];
    let mut closed_paths = vec![];

    paths.push("ö".to_string());

    while paths.len() > 0{
        let mut paths_new = vec![];
        for path in paths.iter(){
            if let Some(neighbours) = nodes.get(path.split(",").last().unwrap()){
                for neighbour in neighbours{
                    if neighbour == "end"{
                        closed_paths.push(path.to_string() + "," + neighbour);
                    }
                    else if neighbour.to_uppercase() == *neighbour || !path.split(",").any(|x| x == neighbour){
                        paths_new.push(path.to_string() + "," + neighbour);
                    }
                }
            }
        }
        paths = paths_new;
    }
        
    for p in closed_paths.iter(){
        println!("{}", p)
    }
    println!("{}", closed_paths.len())
}

fn parse_input() ->  HashMap<String, Vec<String>>  {
    let mut out : HashMap<String, Vec<String>>  = HashMap::new();
    if let Ok(lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                let strings : Vec<&str> = l.split("-").collect();
                let mut string1 = strings[0];
                let mut string2 = strings[1];
                if string1 == "start"{
                    string1 = "ö";
                }
                if string2 == "start"{
                        string2 = "ö";
                }
                if string1 != "end"{
                    out.entry(string1.to_string())
                    .or_default()
                    .push(string2.to_string()); 
                }
                if string2 != "end"{
                    out.entry(string2.to_string())
                    .or_default()
                    .push(string1.to_string()); 
                }
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
