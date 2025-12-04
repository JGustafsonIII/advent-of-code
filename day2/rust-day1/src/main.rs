use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total_invalid_sum: u64 = 0;

    if let Ok(lines) = read_lines("../test/day2_input.txt") {
        for line in lines.map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            for part in line.split(',') {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }

                let (start_str, end_str) = part
                    .split_once('-')
                    .expect("range must contain '-' between start and end");

                let start: u64 = start_str.parse().expect("start must be a number");

                let end: u64 = end_str.parse().expect("end must be a number");

                for n in start..=end {
                    if is_invalid_id(n) {
                        total_invalid_sum += n;
                    }
                }
            }
        }
    }

    println!("{}", total_invalid_sum);
}

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();

    if s.len() % 2 != 0 {
        return false;
    }

    let half = s.len() / 2;
    let (first, second) = s.split_at(half);

    if first.starts_with('0') {
        return false;
    }

    first == second
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
