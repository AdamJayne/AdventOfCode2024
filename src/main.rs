use services::puzzle::print_result;

pub mod days;
pub mod services;

fn main() {
    // TODO: Cleanup day running selection, find a better way to do this
    // Day 1
    let day1_result = days::day1::main().expect("Unable to run Day1");
    print_result(day1_result);
}
