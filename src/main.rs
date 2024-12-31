use services::puzzle::print_result;

pub mod days;
pub mod services;

fn main() {
    // TODO: Cleanup day running selection, find a better way to do this
    // Day 1
    let day1_result = days::day1::main().expect("Unable to run Day1");
    print_result(1, day1_result);

    let day2_result: Vec<i32> = days::day2::main().expect("Unable to run Day2");
    print_result(2, day2_result);

    let day3_result: Vec<i32> = days::day3::main().expect("Unable to run Day3");
    print_result(3, day3_result);
}
