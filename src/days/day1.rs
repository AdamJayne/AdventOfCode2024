use crate::services::puzzle;
use std::io::{self, BufRead};

pub fn main() -> io::Result<Vec<i32>> {
    let values = puzzle::get_puzzle("day1");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let lines = values.lines();
    for line in lines.flatten() {
        let thing: Vec<&str> = line.split_whitespace().collect();
        if let [one, two] = thing.as_slice() {
            left.push(one.parse::<i32>().unwrap());
            right.push(two.parse::<i32>().unwrap());
        }
    }
    left.sort();
    right.sort();

    let mut results: Vec<i32> = Vec::new();

    let part1_result: i32 = part1(left.clone(), right.clone())?;
    let part2_result = part2(left.clone(), right.clone())?;

    results.push(part1_result);
    results.push(part2_result);

    Ok(results)
}

fn part1(left_list: Vec<i32>, right_list: Vec<i32>) -> io::Result<i32> {
    let combined = left_list.iter().zip(right_list);
    let mut result = 0;
    for (l, r) in combined {
        result += (l - r).abs();
    }
    Ok(result)
}

fn part2(left_list: Vec<i32>, right_list: Vec<i32>) -> io::Result<i32> {
    let mut result = 0;
    for l in left_list.iter() {
        let mut found = 0;
        for r in right_list.iter() {
            if l == r {
                found += 1;
            }
        }
        result += l * found;
    }
    Ok(result)
}
