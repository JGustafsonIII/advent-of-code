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
    if rows == 0 {
        println!("0");
        return;
    }
    let cols = grid[0].len();

    let mut total_removed: u64 = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

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

                        if nr >= 0
                            && nr < rows as isize
                            && nc >= 0
                            && nc < cols as isize
                            && grid[nr as usize][nc as usize] == '@'
                        {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove {
            grid[r][c] = '.';
            total_removed += 1;
        }
    }

    println!("{}", total_removed);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
