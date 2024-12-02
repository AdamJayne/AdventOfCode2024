use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn get_puzzle(filename: &str) -> BufReader<File> {
    let mut file_path = PathBuf::from(format!("puzzles/{filename}"));
    file_path.set_extension("txt");
    let file = File::open(file_path).expect(&format!("Unable to open file: {filename}"));
    let reader = BufReader::new(file);
    return reader;
}

pub fn print_result(day: i32, answers: Vec<i32>) {
    let sliced = answers.as_slice();
    println!("Day {:?}:", day);
    println!("    Part1: {}", sliced[0]);
    println!("    Part2: {}", sliced[1]);
    println!("");
}
