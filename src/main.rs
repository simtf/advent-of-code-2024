use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4_1;
mod day_4_2;
mod day_5;
mod day_6_1;
mod day_6_2;
mod day_7;
mod day_8;
mod day_9_1;
mod day_9_2;
mod day_10;

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
            "2" => day_4_2::execute(),
            _ => println!("Day {day} part {part} is not implemented !"),
        },
        "5" => day_5::execute(),
        "6" => match part {
            "1" => day_6_1::execute(),
            "2" => day_6_2::execute(),
            _ => println!("Day {day} part {part} is not implemented !"),
        },
        "7" => day_7::execute(),
        "8" => day_8::execute(),
        "9" => match part {
            "1" => day_9_1::execute(),
            "2" => day_9_2::execute(),
            _ => println!("Day {day} part {part} is not implemented !"),
        },
        "10" => day_10::execute(),
        _ => println!("Day {day} is not implemented !"),
    }
}
