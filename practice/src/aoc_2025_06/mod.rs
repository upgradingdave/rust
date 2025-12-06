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

pub fn part2(path_to_file: &str) -> Result<u64, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    
    let mut cols: HashMap<u64, Vec<String>> = HashMap::new();
    let mut answers: Vec<u64> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;

        let re = Regex::new(r"\s+").unwrap();
        let parts: Vec<&str> = re.split(&line).filter(|s| !s.is_empty()).collect();

        for (i, part) in parts.iter().enumerate() {
            
            let col = cols.entry(i as u64).or_insert_with(Vec::new);
            
            if let Ok(_) = part.parse::<u64>() {
                //println!("Pushing {} to column {}", part, i);
                col.push(part.to_string().chars().rev().collect::<String>());
            } else {

                let mut total:u64 = 0;
                let binding = String::new();
                let longest = col.iter().max_by_key(|s| s.len()).unwrap_or(&binding);
                for l in 0..longest.len() {
                    let chars = col.iter().map(|s| s.chars().nth(l).unwrap_or(' ')).collect::<Vec<char>>();
                    let chars_str = chars.into_iter().collect::<String>().trim().to_string();
                    if let Ok(num) = chars_str.parse::<u64>() {

                        println!("Parsed {} from column {}", num, i);
                        
                        if part.starts_with("+") {
                            total += num;
                        } else if part.starts_with("*") {
                            total *= num;
                        }                        
                    }
                }
                answers.push(total);
            }
        }
    }
    
    // sum the answers
    let answer = answers.iter().sum();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    #[test]
    fn max_str_by_length() {
        let mut strs: Vec<String> = Vec::new();
        strs.push("123".to_string());
        strs.push("1234".to_string());
        strs.push("12345".to_string());
        
        // find the longest string 
        let binding = String::new();
        let longest = strs.iter().max_by_key(|s| s.len()).unwrap_or(&binding);
        
        assert_eq!(longest, &"12345");
    }

}