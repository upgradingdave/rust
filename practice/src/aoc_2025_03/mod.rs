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
        
        for i in 0..line.len()-1 {
            let digit = line[i..=i].parse::<i32>().unwrap_or(0);
            if digit > digit1 {
                digit1 = digit;
                i_digit1 = i;
            }
        }
        
        let mut digit2 = 0;
        
        for j in i_digit1..line.len() {
            let digit = line[j..=j].parse::<i32>().unwrap_or(0);
            if digit > digit2 && j != i_digit1 {
                digit2 = digit;
            }
        }
        
        let digit_str = digit1.to_string() + &digit2.to_string();
        let answer1 = digit_str.parse::<i32>().unwrap_or(0);
        println!("Answer: {}", answer1);
        answer += answer1;
        
    }

    Ok(answer)
}
