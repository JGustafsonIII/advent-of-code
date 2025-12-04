use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_path = "../test/day3_input.txt";

    let mut total_output: u64 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines.map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let bank_max = max_bank_joltage(line);
            total_output += bank_max as u64;
        }
    } else {
        eprintln!("Failed to open input file: {}", input_path);
        return;
    }

    println!("{}", total_output);
}

fn max_bank_joltage(s: &str) -> u32 {
    let digits: Vec<u32> = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).expect("expected digit 0-9"))
        .collect();

    let n = digits.len();
    if n < 2 {
        return 0;
    }

    let mut best: u32 = 0;
    let mut best_suffix: u32 = 0;

    for i in (0..n).rev() {
        let d = digits[i];

        if best_suffix > 0 {
            let value = d * 10 + best_suffix;
            if value > best {
                best = value;
            }
        }

        if d > best_suffix {
            best_suffix = d;
        }
    }

    best
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
