use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn part1(input: &str) -> io::Result<u64> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut splits = 0;
    let mut tachyon_manifolds = reader.lines().collect::<io::Result<Vec<String>>>()?;

    for l in 0..tachyon_manifolds.len() {
        for i in 0..tachyon_manifolds[l].len() {
            let char = tachyon_manifolds[l].chars().nth(i).unwrap();
            let above = if l > 0 {
                tachyon_manifolds[l - 1].chars().nth(i)
            } else {
                None
            };

            println!("[{},{}] ", l, i);
            println!("{:?} ", above);
            println!("{:?}", char);

            if above == Some('S') {
                tachyon_manifolds[l].replace_range(i..i + 1, "|");
            }

            if above == Some('|') {
                if char == '^' {
                    println!("found split");
                    splits += 1;
                    tachyon_manifolds[l].replace_range(i - 1..i, "|");
                    tachyon_manifolds[l].replace_range(i + 1..i + 2, "|");
                } else {
                    tachyon_manifolds[l].replace_range(i..i + 1, "|");
                }
            }
        }
    }

    Ok(splits)
}
