use std::env;

mod day_1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: &str = args.get(1).expect("You should specify the day to execute.");

    match day {
        "1" => day_1::execute(),
        _ => println!("Day {day} is not implemented !"),
    }
}
