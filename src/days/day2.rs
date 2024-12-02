use crate::services::puzzle::get_puzzle;
use std::{
    fs::File,
    i32,
    io::{self, BufRead, BufReader, Lines},
};

pub fn main() -> io::Result<Vec<i32>> {
    let values: BufReader<File> = get_puzzle("day2");
    let mut entries: Vec<Vec<i32>> = Vec::new();
    let lines: Lines<BufReader<File>> = values.lines();
    for line in lines.flatten() {
        let entry: Vec<&str> = line.split_whitespace().collect();
        let converted: Vec<i32> = entry.iter().map(|&x| x.parse::<i32>().unwrap()).collect();
        entries.push(converted);
    }

    let mut results: Vec<i32> = Vec::new();

    let part1_result: i32 = part1(entries.clone())?;
    let part2_result: i32 = part2(entries.clone())?;

    results.push(part1_result);
    results.push(part2_result);

    Ok(results)
}

fn part1(entries: Vec<Vec<i32>>) -> io::Result<i32> {
    let mut result = 0;
    for entry in entries.iter() {
        let mut max_increase: i32 = 0;
        let mut max_decrease: i32 = 0;
        let mut found_no_change: bool = false;
        let mut previous_value: i32 = -1;
        for value in entry.iter() {
            if previous_value == -1 {
                previous_value = *value;
                continue;
            }
            let calculation = value - previous_value;
            if calculation == 0 {
                found_no_change = true;
            } else if calculation > max_increase {
                max_increase = calculation;
            } else if calculation < max_decrease {
                max_decrease = calculation;
            }
            previous_value = *value;
        }
        println!("{}, {}, {}", found_no_change, max_decrease, max_increase);
        if !found_no_change
            && ((max_increase <= 3 && max_decrease == 0)
                || (max_decrease >= -3 && max_increase == 0))
        {
            result += 1;
        }
    }
    Ok(result)
}

fn part2(entries: Vec<Vec<i32>>) -> io::Result<i32> {
    let mut result: i32 = 0;
    Ok(result)
}
