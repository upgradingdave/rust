use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_ranges(path_to_file: &str) -> Result<HashMap<u64, u64>, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    
    let mut ranges: HashMap<u64,u64> = HashMap::new();
    
    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        let mut overlap_found = false;
        let mut new_start = 0;
        let mut new_end = 0;
        let mut overlaps: Vec<u64> = Vec::new();
        for (k, v) in ranges.iter() {
            if start >= *k && start <= *v || end >= *k && end <= *v || start <= *k && end >= *v || start >= *k && end <= *v {
                overlap_found = true;
                //println!("Overlap found {} - {}", k, v);
                if new_start > start.min(*k) || new_start == 0 {
                    new_start = start.min(*k);
                }
                if new_end < end.max(*v) || new_end == 0 {
                    new_end = end.max(*v);
                }
                
                overlaps.push(*k);
            }
        }
        if overlap_found {
            for overlap in overlaps {
                ranges.remove(&overlap);
            }
            ranges.insert(new_start, new_end);
            //println!("Merged {} - {}", new_start, new_end);
        } else {
            ranges.insert(start, end);
            //println!("New range {} - {}", start, end);
        }
    }
    
    return Ok(ranges);
}

pub fn part1(path_to_file: &str) -> Result<u64, io::Error> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);

    let ranges = parse_ranges(path_to_file).unwrap();

    let mut part1 = 0;
    let mut skip_ranges = true;
    
    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            skip_ranges = false;
            continue;
        }
        
        if !skip_ranges {
            let ingredient_id = line.parse::<u64>().unwrap();
            //println!("ingredient: {}", ingredient_id);
            for (k, v) in ranges.iter() {
                if ingredient_id >= *k && ingredient_id <= *v {
                    //println!("Ingredient {} is in range {} - {}", ingredient_id, k, v);
                    part1 += 1;
                    //break;
                }
            }
        }
        
    }
    
    return Ok(part1);
}

pub fn part2(path_to_file: &str) -> Result<u64, io::Error> {
    let ranges = parse_ranges(path_to_file).unwrap();

    let mut part2 = 0;
    for (k, v) in ranges.iter() {
        part2 += v - k + 1;
    }

    return Ok(part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/2025_05a.txt").unwrap(), 3);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2("input/2025_05a.txt").unwrap(), 14);
    }
}
