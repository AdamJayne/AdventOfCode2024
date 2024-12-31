use crate::services::puzzle::{self, get_puzzle};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub fn main() -> io::Result<Vec<i32>> {
    let values: BufReader<File> = get_puzzle("day3");
    let lines: Vec<String> = values.lines().collect::<Result<_, _>>().unwrap();

    let mut results: Vec<i32> = Vec::new();

    let part1_result: i32 = part1(lines)?;
    let part2_result: i32 = part2()?;

    results.push(part1_result);
    results.push(part2_result);

    Ok(results)
}

fn part1(lines: Vec<String>) -> io::Result<i32> {
    let capture_pattern = Regex::new(r"mul\((?<first>\d{1,3})\,(?<second>\d{1,3})\)").unwrap();
    let mut result = 0;

    for line in lines {
        capture_pattern.captures_iter(&line).for_each(|captures| {
            let first = captures
                .name("first")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let second = captures
                .name("second")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            result += first * second;
        });
    }

    Ok(result)
}

fn part2() -> io::Result<i32> {
    Ok(0)
}
