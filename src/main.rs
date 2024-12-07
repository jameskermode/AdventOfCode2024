use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} DAY", args[0]);
        std::process::exit(1);
    }
    let day: u32 = args[1].parse().expect("Cannot parse day number");
    let result = match day {
        1 => day1::run("input/day1.txt").unwrap(),
        2 => day2::run("input/day2.txt").unwrap(),
        3 => day3::run("input/day3.txt").unwrap(),
        4 => day4::run("input/day4.txt").unwrap(),
        5 => day5::run("input/day5.txt").unwrap(),
        6 => day6::run("input/day6.txt").unwrap(),
        _ => {
            eprintln!("Day {day} not yet implemented.");
            std::process::exit(1);
        }
    };
    println!("Result: {:?}", result);
}
