use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_path = "../test/day3_input.txt";

    let mut total_output: u128 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines.map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let bank_max = max_bank_joltage_k(line, 12);
            total_output += bank_max;
        }
    } else {
        eprintln!("Failed to open input file: {}", input_path);
        return;
    }

    println!("{}", total_output);
}

fn max_bank_joltage_k(s: &str, k: usize) -> u128 {
    let digits: Vec<u8> = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).expect("expected digit 0-9") as u8)
        .collect();

    let n = digits.len();
    if k == 0 || n == 0 {
        return 0;
    }

    if n <= k {
        let mut val: u128 = 0;
        for d in digits {
            val = val * 10 + d as u128;
        }
        return val;
    }

    let mut to_drop = n - k;
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    for d in digits {
        while to_drop > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            to_drop -= 1;
        }
        stack.push(d);
    }

    stack.truncate(k);

    let mut result: u128 = 0;
    for d in stack {
        result = result * 10 + d as u128;
    }
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
