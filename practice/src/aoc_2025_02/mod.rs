// https://adventofcode.com/2025/day/2
use std::fs;

pub fn has_repetition(i_str: &str, num_chunks: &usize) -> bool {
    let str_len = i_str.len();
    if str_len % num_chunks == 0 {
        let chunk_size = str_len / num_chunks;
        let first_chunk = &i_str[0..chunk_size];
        for k in 1..str_len / chunk_size {
            let next_chunk = &i_str[k*chunk_size..(k*chunk_size)+chunk_size];
            //println!("Comparing {} and {}", first_chunk, next_chunk);
            if first_chunk != next_chunk {
                return false;
            }
        }
        return true;
    } else {
        return false;
    }
}

pub fn part1(path: &str) -> Result<u64, std::io::Error> {
    let input = fs::read_to_string(path)?;
    let mut answer = 0;

    for range in input.split(',') {
        println!("processing range {}", range);
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap();
        let end = parts[1].parse::<u64>().unwrap();

        for i in start..=end {
            let i_str = i.to_string();
            let str_len = i_str.len();
            if str_len % 2 == 0 {
                let half_len = str_len / 2;
                let first_half = &i_str[0..half_len];
                let second_half = &i_str[half_len..];
                
                if first_half == second_half {
                    answer += i;
                }
            }
        }
    }
    Ok(answer)
}

pub fn part2(path: &str) -> Result<u64, std::io::Error> {
    let input = fs::read_to_string(path)?;
    let mut answer = 0;

    for range in input.split(',') {
        //println!("processing range {}", range);
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap();
        let end = parts[1].parse::<u64>().unwrap();

        for i in start..=end {
            let i_str = i.to_string();
            let str_len = i_str.len();

            let mut match_found = false;
            
            for j in 2..=str_len {
                if has_repetition(&i_str, &j) {
                    match_found = true;
                    break;
                }
            }

            if match_found {
                //println!("Found match for {}", i);
                answer += i;
            }
        }
    }
    Ok(answer)
}
