#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    
    let (commands, boards)= parse_input();

    let mut rows = vec![vec![0; 5]; boards.len()];
    let mut cols = vec![vec![0; 5]; boards.len()];
    let mut winning_commands = vec![];
    let mut winning_board = vec![];
    let mut winning_number = 0;

    for command in commands.split(","){
        let comm = command.parse::<u32>().unwrap();
        for (i,board) in boards.iter().enumerate(){
            if let Some(idx) = board.iter().position(|x| x == &comm){
                let mut has_won = false;
                if let Some(&mx) = rows[i].iter().max(){
                    if let Some(&mx2) = cols[i].iter().max(){
                        if mx == 5 || mx2 == 5 {
                            has_won = true;
                        }
                    }
                }
                rows[i][idx/5] += 1;
                cols[i][idx%5] += 1;

                if (rows[i][idx/5] == 5 || cols[i][idx%5] == 5) && !has_won {
                    winning_board = board.to_vec();
                    winning_number = comm;
                }
            }
        }
    }
    for command in commands.split(","){
        let comm = command.parse::<u32>().unwrap();
        winning_commands.push(comm);
        if comm == winning_number{
            break;
        }
    }
    

    let winning_board_unused_sum : u32 = winning_board.iter().filter(|x| winning_commands.iter().find(|y| y == x).is_none()).sum();



    println!("{}", winning_board_unused_sum*winning_number)
}

fn parse_input() -> (String, Vec::<Vec::<u32>>) {
    let mut out = vec![];
    let input : String;
    //let mut instr = "";

    if let Ok(mut lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        input = lines.next().unwrap().unwrap();
        while let Some(Ok(_line)) = lines.next() {
            let mut outt : Vec<u32> = vec![];
            for _ in 0..5{
                let z = lines.next().unwrap().unwrap();

                let mut app : Vec<u32> = z.split(" ").filter(|x| x != &"").map(|x| x.parse::<u32>().unwrap()).collect();
                outt.append(&mut app);
            }
            out.push(outt);
        }
    }
    else{
        input = "".to_string();
    }
    return (input, out);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
