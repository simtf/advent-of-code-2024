use regex::Regex;
use std::fs;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_3.txt").unwrap();
    let lines: Vec<&str> = dataset.split("\r\n").collect();

    let re: Regex = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let mut sum: usize = 0;
    let mut activated: bool = true;

    for line in lines {
        let instructions: Vec<&str> = re.find_iter(line).map(|f| f.as_str()).collect();
        for instruction in instructions {
            if instruction == "do()" {
                activated = true;
            } else if instruction == "don't()" {
                activated = false;
            } else if activated {
                let mul: String = instruction.replace("mul(", "");
                let mul: String = mul.replace(")", "");
                let digits: (&str, &str) = mul.split_once(",").unwrap();
                sum += digits.0.parse::<usize>().unwrap() * digits.1.parse::<usize>().unwrap();
            }
        }
    }
    println!("Sum: {sum}");
}
