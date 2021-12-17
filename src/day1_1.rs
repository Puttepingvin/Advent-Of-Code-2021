use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut n = -1;
    let mut last = -1;
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Ok(i) = ip.parse::<i32>() {
                    if i > last{
                        n+=1;
                    }
                    last = i;
                }
            }
        }
    }
    println!("{}", n)
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
