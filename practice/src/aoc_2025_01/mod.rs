// https://adventofcode.com/2025/day/1
use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum Rotations {
    LEFT(i32),
    RIGHT(i32),
}

pub fn read_rotations_from_file() -> io::Result<()> {
    let file = File::open("input/2025_01.txt")?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    
    Ok(())
}
