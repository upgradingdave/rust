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
    
    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let mut col_meta: Vec<String> = Vec::new();
    let mut part = String::new();
    for (i, c) in lines[lines.len()-1].chars().enumerate() {
        
        let peek = lines[lines.len()-1].chars().nth(i+1);
        if peek == Some('+') || peek == Some('*') {
            col_meta.push(part);
            part = String::new();
        } else {
            part.push(c);
        }
    }
    col_meta.push(part);
    
    let mut answers: Vec<u64> = Vec::new();
    
    let mut offset = 0;
    for meta in col_meta {
        println!("meta: {}", meta);
        let col_len = meta.len();
        let mut s = String::new();
        let mut col:Vec<String> = Vec::new();
        for j in offset..col_len+offset {
            for i in 0..lines.len()-1 {
                s += lines[i].chars().nth(j).unwrap().to_string().as_str();
            }        
            println!("s: {}", s);
            col.push(s);
            s = String::new();
        }
        offset += col_len+1;

        if meta.starts_with('+') {
            answers.push(col.iter().map(|s| s.trim().parse::<u64>().unwrap_or(0)).sum::<u64>());            
        } else if meta.starts_with('*') {
            answers.push(col.iter().map(|s| s.trim().parse::<u64>().unwrap_or(0)).product::<u64>());            
        }
    }

    println!("answers: {:?}", answers);
    Ok(answers.iter().sum::<u64>())
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