// https://adventofcode.com/2025/day/1
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Rotations {
    LEFT(i32),
    RIGHT(i32),
    INVALID
}

pub fn read_rotations_from_file(path_to_file: &str) -> Result<i32, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    let mut current = 50;
    let mut answer = 0;
    
    for line in reader.lines() {
        let line = line?;
        parse_line(&line);

        let r = parse_line(&line);
        current = *rotate(&mut current, &r);
        
        //println!("Rotate {:?} to point at {}", &r, current);

        if current == 0 {
            answer += 1;
        }

    }
    Ok(answer)
}

fn rotate<'a>(current: &'a mut i32, rotation: &Rotations) -> &'a mut i32 {
    match rotation {
        Rotations::LEFT(n) => {
            *current -= n;
            match current {
                x if *x < 0 => {
                    *x += 100;
                    x
                },
                _ => current,
            }
        },
        Rotations::RIGHT(n) => {
           *current += n;
            match current {
                x if *x > 99 => {
                    *x -= 100;
                    x
                },
                _ => current,
            }
   
        },
        Rotations::INVALID => {
            current
        },
    }
}

fn parse_line(line: &str) -> Rotations {
    let c = line.chars().nth(0).unwrap();
    let n = line[1..].parse::<i32>().unwrap();
    
    match c {
        'L' => Rotations::LEFT(n),
        'R' => Rotations::RIGHT(n),
        _ => Rotations::INVALID,
    }
}

