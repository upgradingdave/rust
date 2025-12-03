use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn part1(path_to_file: &str) -> Result<i32, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);

    let mut answer = 0;

    for line in reader.lines() {
        let line = line?;
        //println!("Line: {}", line);

        let mut digit1 = 0;
        let mut i_digit1 = 0;

        for i in 0..line.len() - 1 {
            let digit = line[i..=i].parse::<i32>().unwrap();
            if digit > digit1 {
                digit1 = digit;
                i_digit1 = i;
            }
        }

        let mut digit2 = 0;

        for j in i_digit1..line.len() {
            let digit = line[j..=j].parse::<i32>().unwrap();
            if digit > digit2 && j != i_digit1 {
                digit2 = digit;
            }
        }

        let digit_str = digit1.to_string() + &digit2.to_string();
        let answer1 = digit_str.parse::<i32>().unwrap();
        println!("Answer: {}", answer1);
        answer += answer1;
    }

    Ok(answer)
}

pub fn joltage(line: &str, start: u32, needed: u32) -> (u32, u32) {
    let mut max: u32 = 0;
    let mut i_max = 0;

    let end = line.len() as u32 - needed;
    //println!("Start {} End {}", start, end);

    for i in start..end {
        let digit = line
            .chars()
            .nth(i as usize)
            .unwrap()
            .to_digit(10)
            .unwrap();

        if digit > max {
            max = digit;
            i_max = i;
        }
    }

    //println!("max: {} found at idx: {}", max, i_max);

    (i_max, max)
}

pub fn part2(path_to_file: &str) -> Result<u64, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    
    let mut answer:u64 = 0;

    for line in reader.lines() {
        
        let line = line?;
        let mut largest = String::from("");
        let mut start = 0;
        
        for needed in (0..12).rev() {
            let max;
            (start, max) = joltage(&line, start, needed);
            largest = largest + &max.to_string();
            start = start + 1;
        }
        
        //println!("Largest: {:?}", largest);
        answer += largest.parse::<u64>().unwrap();
        
    }

    Ok(answer)
}
