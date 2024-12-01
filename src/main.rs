use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} DAY", args[0]);
        std::process::exit(1);
    }
    let day: u32 = args[1].parse().expect("Cannot parse day number");
    let result = match day {
        1 => day1::run("input/day1.txt").unwrap(),
        _ => {
            eprintln!("Day {day} not yet implemented.");
            std::process::exit(1);
        }
    };
    println!("Result: {:?}", result);
}
