use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {
    println!("Day 1");
    day01::main();

    println!("\nDay 2");
    day02::main();

    println!("\nDay 3");
    day03::main();

    println!("\nDay 4");
    day04::main();

    println!("\nDay 5");
    day05::main();

    println!("\nDay 6");
    day06::main();

    println!("\nDay 7");
    day07::main();

    println!("\nDay 8");
    day08::main();

    println!("\nDay 9");
    day09::main();

    println!("\nDay 10");
    day10::main();

    println!("\nDay 11");
    day11::main();

    println!("\nDay 12");
    day12::main();

    println!("\nDay 13");
    day13::main();

    println!("\nDay 14");
    day14::main();

    println!("\nDay 15");
    day15::main();
}

pub fn get_input(day: u8, test: bool) -> String {
    let mut input_file = "input";
    if test {
        input_file = "test_input";
    }
    let path = format!("src/day{day:0>2}/{input_file}");
    fs::read_to_string(path).expect("Should have been able to read the file.")
}
