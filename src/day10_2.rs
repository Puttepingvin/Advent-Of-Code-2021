#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {


    let lines = parse_input();

    let mut closers = HashMap::new();
    closers.insert('[', ']');
    closers.insert('(', ')');
    closers.insert('<', '>');
    closers.insert('{', '}');


    
    let mut scores = HashMap::new();
    scores.insert(']', 2);
    scores.insert(')', 1);
    scores.insert('}', 3);
    scores.insert('>', 4);

    let mut score = vec![];


    for line in lines{
        let mut expected = vec![];
        let mut corrupted = false;
        //let mut incopmplete = false;
        //println!("{}", line);
        for c in line.chars(){
            match closers.get(&c){
                Some(closer) => expected.push(closer.clone()),
                None => {
                    match expected.pop(){
                        Some(closer) => {
                            if c != closer{
                                corrupted = true;
                                //score += scores.get(&c).unwrap();
                                //println!("Corrupted");
                                break;
                            }
                            else{
                                //Just closing normally
                            }
                        }
                        None => {
                            //Too many closers
                        }
                    }
                }
            }
        }
        if !corrupted{
            let mut score_local : u64 = 0;
            expected.reverse();
            println!("{}", expected.iter().collect::<String>());
            for e in &expected{
                score_local *= 5;
                score_local += scores.get(&e).unwrap();
            }
            score.push(score_local);
        }
    }
    
    score.sort();
    println!("{}", score[(score.len()-1) / 2])
}

fn parse_input() ->  Vec::<String> {
    let mut out = vec![];
    if let Ok(lines) = read_lines("./input") {
        for row in lines{
            if let Ok(l) = row{
                out.push(l);
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
