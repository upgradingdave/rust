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
    
    let mut accessible = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            //println!("Grid[{}][{}] is '@': {}", i, j, grid[i][j]);
            if grid[i][j] {
              let left_neighbor = i > 0 && grid[i - 1][j];
              let right_neighbor = i < grid.len() - 1 && grid[i + 1][j];
              let top_neighbor = j > 0 && grid[i][j - 1];
              let bottom_neighbor = j < grid[i].len() - 1 && grid[i][j + 1];
              let left_top_neighbor = i > 0 && j > 0 && grid[i - 1][j - 1];
              let right_top_neighbor = i > 0 && j < grid[i].len() - 1 && grid[i - 1][j + 1];
              let left_bottom_neighbor = i < grid.len() - 1 && j > 0 && grid[i + 1][j - 1];
              let right_bottom_neighbor = i < grid.len() - 1 && j < grid[i].len() - 1 && grid[i + 1][j + 1];
              
              let mut count = 0;
              if left_neighbor { count += 1; }
              if right_neighbor { count += 1; }
              if top_neighbor { count += 1; }
              if bottom_neighbor { count += 1; }
              if left_top_neighbor { count += 1; }
              if right_top_neighbor { count += 1; }
              if left_bottom_neighbor { count += 1; }
              if right_bottom_neighbor { count += 1; }
              
              if count < 4 {
                  accessible = accessible + 1;
              }
            }
        }
    }    
    Ok(accessible)
}
