// https://adventofcode.com/2025/day/1
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_rotations_from_file(path_to_file: &str) -> Result<(i32, i32), io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    let mut current = 50;
    let mut answer1 = 0;
    let mut answer2 = 0;
    
    for line in reader.lines() {
        let line = line?;
        let dir = line.chars().nth(0).unwrap();
        let steps = line[1..].parse::<i32>().unwrap();
        
        for _ in 0..steps {
            match dir {
                'L' => current = if current - 1 < 0 { 99 } else { current - 1 },
                'R' => current = (current + 1) % 100,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid direction")),
            }
            
            if current == 0 {
                answer2 += 1;
            }
        }
        
        //println!("Rotate {}{} to point at {}", dir, steps, current);

        if current == 0 {
            answer1 += 1;
        }

    }
    
    Ok((answer1, answer2))
}
