use std::fs;

fn parse_grid(filename: &str) -> Result<Vec<Vec<bool>>, std::io::Error> {
    let content = fs::read_to_string(filename)?;
    
    let grid: Vec<Vec<bool>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch == '@')  // true for '@', false for '.'
                .collect()
        })
        .collect();
    
    Ok(grid)
}

pub fn part1(file: &str) -> Result<u64, std::io::Error> {
    let grid = parse_grid(file)?;
    
    println!("Grid[0][2] is '@': {}", grid[0][2]);
    
    Ok(0)
}
