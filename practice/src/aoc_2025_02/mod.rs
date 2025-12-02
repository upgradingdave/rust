// https://adventofcode.com/2025/day/2
use std::fs;

pub fn read_file(path: &str) -> Result<i32, std::io::Error> {
    let input = fs::read_to_string(path)?;
    
    for range in input.split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap();
        let end = parts[1].parse::<u64>().unwrap();
        
        if start <= end {
            println!("Range: {}-{}", start, end);
        } else {
            println!("Invalid range: {}-{}", start, end);
        }
        
    }
    //println!("{}", content);
    Ok(1)
}
