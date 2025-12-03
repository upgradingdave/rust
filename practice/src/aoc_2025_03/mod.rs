use std::f64::INFINITY;
use std::fs::File;
use std::i32::MAX;
use std::io::{self, BufRead, BufReader};

pub fn part1(path_to_file: &str) -> Result<i32, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    let mut max = 0;

    for line in reader.lines() {
        let line = line?;
        println!("Line: {}", line);

        
        for i in 0..line.len() {
            let digit = line[i..=i].parse::<i32>().unwrap_or(0);
            if digit > max {
                max = digit;
            }
        }
    }

    Ok(max)
}
