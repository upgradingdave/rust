use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

pub fn part1(path_to_file: &str) -> Result<u64, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    
    let mut cols: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut answers: Vec<u64> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;

        let re = Regex::new(r"\s+").unwrap();
        let parts: Vec<&str> = re.split(&line).filter(|s| !s.is_empty()).collect();
        
        println!("Processing line: {}", line);
        println!("Parts: {:?}", parts);
        println!("Length: {}", parts.len());
        
        for (i, part) in parts.iter().enumerate() {
            println!("Processing part {}", part);
            let col = cols.entry(i as u64).or_insert_with(Vec::new);
            
            if let Ok(num) = part.parse::<u64>() {
                println!("Pushing {} to column {}", num, i);
                col.push(num);
            } else if part.starts_with("+") {
                let sum = col.iter().sum();
                println!("Sum of column {}: {}", i, sum);
                answers.push(sum);
            } else if part.starts_with("*") {
                let product = col.iter().product();
                println!("Product of column {}: {}", i, product);
                answers.push(product);
            }
        }
    }
    
    // sum the answers
    let answer = answers.iter().sum();
    Ok(answer)
}