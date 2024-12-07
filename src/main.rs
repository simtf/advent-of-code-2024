use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4_1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: &str = args.get(1).expect("You should specify the day to execute.");
    let part: &str = args.get(2).map(|s: &String| s.as_str()).unwrap_or("");

    match day {
        "1" => day_1::execute(),
        "2" => day_2::execute(),
        "3" => day_3::execute(),
        "4" => match part {
            "1" => day_4_1::execute(),
            _ => println!("Day {day} part {part} is not implemented !"),
        },
        _ => println!("Day {day} is not implemented !"),
    }
}
