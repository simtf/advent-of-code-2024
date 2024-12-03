use std::env;

mod day_1;
mod day_2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: &str = args.get(1).expect("You should specify the day to execute.");

    match day {
        "1" => day_1::execute(),
        "2" => day_2::execute(),
        _ => println!("Day {day} is not implemented !"),
    }
}
