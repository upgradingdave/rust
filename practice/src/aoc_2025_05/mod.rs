use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn part1(path_to_file: &str) -> Result<u64, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);

    let mut parse_range_mode = true;
    
    let mut ranges: HashMap<u64,u64> = HashMap::new();
    
    let mut answer = 0;
    
    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            parse_range_mode = false;
            continue;
        }
        
        if parse_range_mode {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            let mut overlap_found = false;
            let mut new_start = 0;
            let mut new_end = 0;
            let mut old_start = 0;
            for (k, v) in ranges.iter() {
                if start >= *k && start <= *v || end >= *k && end <= *v || start <= *k && end >= *v || start >= *k && end <= *v {
                    overlap_found = true;
                    old_start = *k;
                    new_start = start.min(*k);
                    new_end = end.max(*v);
                    break;
                }
            }
            if overlap_found {
                ranges.remove(&old_start);
                ranges.insert(new_start, new_end);
                println!("Overlap found {} - {}", new_start, new_end);
            } else {
                ranges.insert(start, end);
                println!("New range {} - {}", start, end);
            }
        } else {
            let ingredient_id = line.parse::<u64>().unwrap();
            println!("ingredient: {}", ingredient_id);
            for (k, v) in ranges.iter() {
                if ingredient_id >= *k && ingredient_id <= *v {
                    println!("Ingredient {} is in range {} - {}", ingredient_id, k, v);
                    answer += 1;
                    break;
                }
            }
        }

    }
    
    return Ok(answer);
}
