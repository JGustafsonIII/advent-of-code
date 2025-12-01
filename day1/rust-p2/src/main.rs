use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let mut times_at_zero: i32 = 0;
    let mut dial = 50;

    if let Ok(lines) = read_lines("../test_data/example.txt") {
        for line in lines.map_while(Result::ok) {
            if line.starts_with("L") {
                let line_slice: &str = &line[1..];
                let temp: i32 = line_slice.parse().expect("Failed to parse string to i32");

                let first_distance = if dial > 0 { dial } else { 100 };
                let wraps = if temp < first_distance {
                    0
                } else {
                    1 + (temp - first_distance) / 100
                };
                times_at_zero += wraps;

                dial = (dial - temp).rem_euclid(100);
            } else if line.starts_with("R") {
                let line_slice: &str = &line[1..];
                let temp: i32 = line_slice.parse().expect("Failed to parse string to i32");

                let wraps = (dial + temp) / 100;
                times_at_zero += wraps;

                dial = (dial + temp).rem_euclid(100);
            }
        }
    }
    println!("{}", times_at_zero);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
