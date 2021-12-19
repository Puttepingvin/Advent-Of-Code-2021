#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {


    let (mut keys, nums) = parse_input();
    let mut sum = 0;

    for (i,k_arr) in keys.iter_mut().enumerate(){
        let mut codes = vec!["".to_string();10];
        codes[1] = k_arr.iter().find(|k| k.len() == 2).unwrap().clone();
        k_arr.retain(|x| x != &codes[1]);
        codes[7] = k_arr.iter().find(|k| k.len() == 3).unwrap().clone();
        k_arr.retain(|x| x != &codes[7]);
        codes[4] = k_arr.iter().find(|k| k.len() == 4).unwrap().clone();
        k_arr.retain(|x| x != &codes[4]);
        codes[8] = k_arr.iter().find(|k| k.len() == 7).unwrap().clone();
        k_arr.retain(|x| x != &codes[8]);
        codes[3] = k_arr.iter().find(|k| k.len() == 5 && codes[1].chars().all(|c| k.chars().any(|c2| c2 == c))).unwrap().clone();
        k_arr.retain(|x| x != &codes[3]);
        codes[9] = k_arr.iter().find(|k| k.len() == 6 && codes[3].chars().all(|c| k.chars().any(|c2| c2 == c))).unwrap().clone();
        k_arr.retain(|x| x != &codes[9]);
        codes[6] = k_arr.iter().find(|k| k.len() == 6 && codes[1].chars().any(|c| !k.chars().any(|c2| c2 == c))).unwrap().clone();
        k_arr.retain(|x| x != &codes[6]);
        codes[0] = k_arr.iter().find(|k| k.len() == 6).unwrap().clone();
        k_arr.retain(|x| x != &codes[0]);
        codes[5] = k_arr.iter().find(|k| k.chars().all(|c| codes[6].chars().any(|c2| c == c2))).unwrap().clone();
        k_arr.retain(|x| x != &codes[5]);
        codes[2] = k_arr[0].clone();
        for (j, num) in nums[i].iter().enumerate(){
            
            sum += i32::pow(10,3-j as u32)*codes.iter().position(|k| k.chars().all(|c| num.chars().any(|c2| c == c2)) && num.chars().all(|c| k.chars().any(|c2| c == c2))).unwrap() as i32;
        
        }
    }

    println!("{}", sum)
}

fn parse_input() -> (Vec::<Vec::<String>>, Vec::<Vec::<String>>) {
    let mut out = vec![];
    let mut outstr = vec![];
    if let Ok(lines) = read_lines("./input") {
        for row in lines{
            let mut outt = vec![];
            let mut outtstr = vec![];
            if let Ok(l) = row{
                let mut parts = l.split(" | ");
                for seg in parts.next().unwrap().split(" "){
                    outtstr.push(seg.to_string());
                }
                for seg in parts.next().unwrap().split(" "){
                    outt.push(seg.to_string());
                }
            }
            out.push(outt);
            outstr.push(outtstr);
        }
    }
    return (outstr, out);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
