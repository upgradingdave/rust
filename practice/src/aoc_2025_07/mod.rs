use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn solution(input: &str) -> io::Result<(u64, u64)> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut splits = 0;
    let mut lines = reader.lines().collect::<io::Result<Vec<String>>>()?;
    let mut beam_locations = vec![0; lines[0].len()];

    for l in 0..lines.len() {
        for i in 0..lines[l].len() {
            let char = lines[l].chars().nth(i).unwrap();
            let above = if l > 0 {
                lines[l - 1].chars().nth(i)
            } else {
                None
            };

            println!("[{},{}] ", l, i);
            println!("{:?} ", above);
            println!("{:?}", char);

            if above == Some('S') {
                lines[l].replace_range(i..i + 1, "|");
                beam_locations[i] = 1;
            }

            if above == Some('|') {
                if char == '^' {
                    println!("found split");
                    splits += 1;
                    lines[l].replace_range(i - 1..i, "|");
                    beam_locations[i - 1] += beam_locations[i];
                    lines[l].replace_range(i + 1..i + 2, "|");
                    beam_locations[i + 1] += beam_locations[i];
                    beam_locations[i] = 0;
                } else {
                    lines[l].replace_range(i..i + 1, "|");
                }
            }
        }
    }

    // sum up the tachyon manifolds
    let total = beam_locations.iter().sum();

    Ok((splits, total))
}

