use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_path = "../test/input.txt";

    let mut grid: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines.map_while(Result::ok) {
            let trimmed = line.trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            grid.push(trimmed.chars().collect());
        }
    } else {
        eprintln!("Failed take the L: {}", input_path);
        return;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut neighbors = 0;

            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nr < rows as isize &&
                       nc >= 0 && nc < cols as isize &&
                       grid[nr as usize][nc as usize] == '@'
                    {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                accessible_count += 1;
            }
        }
    }

    println!("{}", accessible_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
