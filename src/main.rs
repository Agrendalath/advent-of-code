use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!("Day 1");
    day1::main();

    println!("\nDay 2");
    day2::main();

    println!("\nDay 3");
    day3::main();

    println!("\nDay 4");
    day4::main();

    println!("\nDay 5");
    day5::main();

    println!("\nDay 6");
    day6::main();

    println!("\nDay 7");
    day7::main();
}

pub fn get_input(day: u8, test: bool) -> String {
    let mut input_file = "input";
    if test {
        input_file = "test_input";
    }
    let path = format!("src/day{day}/{input_file}");
    fs::read_to_string(path).expect("Should have been able to read the file.")
}