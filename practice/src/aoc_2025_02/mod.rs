// https://adventofcode.com/2025/day/2
use std::fs;

pub fn read_file(path: &str) -> Result<u64, std::io::Error> {
    let input = fs::read_to_string(path)?;
    let mut answer = 0;

    for range in input.split(',') {
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
